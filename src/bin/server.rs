use std::{error::Error, net::SocketAddr};

use clap::Parser;
use tokio::{
    io::AsyncReadExt,
    join,
    net::{TcpListener, TcpStream},
    spawn,
};
use xo::{Args, XOGame};

const SEARCH_PORT: u16 = 7777;
const CONNECT_PORT: u16 = 8888;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = Args::parse().to_string();

    let listener = TcpListener::bind(addr.clone()).await?;
    println!("listening at {addr}..");

    let mut handles = Vec::new();

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                let handle = spawn(async move {
                    handle_client(socket, addr).await;
                });
                handles.push(handle);
            }
            Err(_) => eprintln!("could not accept client connetion"),
        }
    }
}

async fn handle_client(socket: TcpStream, addr: SocketAddr) {
    println!("client connected from {addr}");
    let (mut reader, writer) = socket.into_split();

    let handle = spawn(async move {
        let mut buf = String::new();
        loop {
            match reader.read_to_string(&mut buf).await {
                Ok(0) | Err(_) => {
                    eprintln!("client {addr} disconnected o7..");
                    break;
                }
                Ok(_) => println!("{addr}: {buf}"),
            }
            buf.clear();
        }
    });
    let _ = join!(handle);
}
