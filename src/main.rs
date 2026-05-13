use std::time::Duration;

use tokio::{
    net::{TcpStream, UdpSocket},
    time::timeout,
};

use crate::xo::XOGame;

mod xo;

#[tokio::main]
async fn main() {
    match find_host().await {
        Some(_) => println!("found a host!"),
        None => println!("no hosts were found!"),
    }
}

struct Room {
    game: XOGame,
}

async fn find_host() -> Option<TcpStream> {
    let search_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

    search_socket.set_broadcast(true).unwrap();

    search_socket
        .send_to(b"PING", "255.255.255.255:7777")
        .await
        .unwrap();

    let mut buf = [0u8; 4];
    match timeout(Duration::from_secs(1), search_socket.recv_from(&mut buf)).await {
        Ok(Ok((_, host_addr))) => {
            if buf == *b"PONG" {
                Some(TcpStream::connect(host_addr).await.unwrap())
            } else {
                None
            }
        }
        _ => None,
    }
}
