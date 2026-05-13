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

    search_socket
        .send_to(b"PING", ("255.255.255.255", SEARCH_PORT))
        .await
        .ok()?;

    let mut buf = [0u8; 4];
    match timeout(Duration::from_secs(1), search_socket.recv_from(&mut buf)).await {
        Ok(Ok((_, host_addr))) => (buf == *b"PONG").then_some(
            TcpStream::connect((host_addr.ip(), CONNECT_PORT))
                .await
                .ok()?,
        ),
        _ => None,
    }
}
