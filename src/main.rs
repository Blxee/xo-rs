use std::time::Duration;

use tokio::{
    net::{TcpStream, UdpSocket},
    time::timeout,
};

use crate::client::client_main;
use crate::host::host_main;

mod client;
mod host;
mod xo;

const SEARCH_PORT: u16 = 7777;
const CONNECT_PORT: u16 = 8888;

#[tokio::main]
async fn main() {
    match find_host().await {
        Some(host_socket) => client_main(host_socket).await,
        None => host_main().await,
    }
}

async fn find_host() -> Option<TcpStream> {
    let search_socket = UdpSocket::bind("0.0.0.0:0").await.ok()?;

    search_socket.set_broadcast(true).ok()?;
    let mut search_port = SEARCH_PORT;

    loop {
        println!("searching for a host in {search_port}..");

        search_socket
            .send_to(b"PING", ("255.255.255.255", search_port))
            .await
            .ok()?;

        let mut buf = [0u8; 4];
        match timeout(Duration::from_secs(1), search_socket.recv_from(&mut buf)).await {
            Ok(Ok((_, host_addr))) => {
                let parse_result = String::from_utf8_lossy(&mut buf).parse::<u16>();

                if let Ok(connect_port) = parse_result {
                    if let Ok(socket) = TcpStream::connect((host_addr.ip(), connect_port)).await {
                        return Some(socket);
                    }
                };

                search_port += 1;
            }
            _ => return None,
        }
    }
}
