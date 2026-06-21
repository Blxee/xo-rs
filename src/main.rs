mod xo;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{io, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin},
    net::{TcpListener, TcpStream, UdpSocket},
    select,
    time::timeout,
};

use crate::xo::XOGame;

#[tokio::main]
async fn main() -> io::Result<()> {
    match find().await {
        Ok(host) => chat(host).await?,
        _ => {
            let game = XOGame::new();
            host(game).await?;
        }
    }

    Ok(())
}

const SEARCH_PORT: u16 = 7777;
const CONNECT_PORT: u16 = 8888;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
enum Protocol {
    Ping,
    Pong,
    Play(u32),
}

impl TryFrom<&[u8]> for Protocol {
    type Error = serde_json::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let s = String::from_utf8_lossy(value);
        serde_json::from_str(&s)
    }
}

async fn find() -> io::Result<TcpStream> {
    println!("searching for host...");
    // make a broadcast socket
    let broadcast_socket = UdpSocket::bind("0.0.0.0:0").await?;
    broadcast_socket.set_broadcast(true)?;

    // broadcast a message to all ips in the network
    broadcast_socket
        .send_to(
            serde_json::to_string(&Protocol::Ping)?.as_bytes(),
            format!("255.255.255.255:{SEARCH_PORT}"),
        )
        .await?;

    println!("waiting for response...");
    let mut buf = vec![];
    // wait 2 seconds for a reply from any host
    let (_, mut host_addr) = timeout(
        Duration::from_secs(2),
        broadcast_socket.recv_buf_from(&mut buf),
    )
    .await??;
    let response = Protocol::try_from(buf.as_slice())?;
    if response != Protocol::Pong {
        todo!()
    }
    println!("host responded, trying to connect...");

    // change the host port to the tcp port and try to connect
    host_addr.set_port(CONNECT_PORT);
    let host_socket = TcpStream::connect(host_addr).await?;

    println!("connected to host");
    Ok(host_socket)
}

async fn host(game: XOGame) -> io::Result<()> {
    println!("hosting a game, waiting for seaching client..");

    let search_socket = UdpSocket::bind(format!("0.0.0.0:{SEARCH_PORT}")).await?;

    let mut buf = vec![];
    loop {
        let (_, addr) = search_socket.recv_buf_from(&mut buf).await?;
        let response = Protocol::try_from(buf.as_slice())?;
        if response == Protocol::Ping {
            search_socket
                .send_to(serde_json::to_string(&Protocol::Pong)?.as_bytes(), addr)
                .await?;
            break;
        }
    }

    println!("client found, trying to establish connection..");
    let listener = TcpListener::bind(format!("0.0.0.0:{CONNECT_PORT}")).await?;

    let (client_socket, _) = listener.accept().await?;

    chat(client_socket).await?;

    Ok(())
}

async fn chat(socket_stream: TcpStream) -> io::Result<()> {
    let peer_addr = socket_stream.peer_addr()?;
    let (reader, mut writer) = socket_stream.into_split();

    let mut read_handle = tokio::spawn(async move {
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();

        while let Ok(n) = buf_reader.read_line(&mut line).await {
            if n == 0 {
                break;
            }
            print!("[{peer_addr}]: {line}");
            line.clear();
        }
    });

    let mut write_handle = tokio::spawn(async move {
        let mut buf_reader = BufReader::new(stdin());
        let mut line = String::new();

        while let Ok(n) = buf_reader.read_line(&mut line).await {
            if n == 0 {
                break;
            }
            let Ok(_) = writer.write_all(line.as_bytes()).await else {
                break;
            };
            line.clear();
        }
    });

    select! {
        _ = &mut read_handle => {
            write_handle.abort();
            write_handle.await?;
        },
        _ = &mut write_handle => {
            read_handle.abort();
            read_handle.await?;
        },
    }
    Ok(())
}
