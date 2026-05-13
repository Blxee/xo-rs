use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, stdin},
    net::{TcpListener, UdpSocket},
};

use crate::{CONNECT_PORT, SEARCH_PORT};

pub(crate) async fn host_main() {
    tokio::spawn(async {
        let search_socket = UdpSocket::bind(("0.0.0.0", SEARCH_PORT)).await.unwrap();

        let mut buf = [0u8; 4];
        while let Ok((_, client_addr)) = search_socket.recv_from(&mut buf).await {
            if buf == *b"PING" {
                search_socket.send_to(b"PONG", client_addr).await.unwrap();
            }
        }
    });

    let listener = TcpListener::bind(("0.0.0.0", CONNECT_PORT)).await.unwrap();

    while let Ok((client_socket, client_addr)) = listener.accept().await {
        println!("connected to client");

        let (mut reader, mut writer) = client_socket.into_split();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            while let Ok(1..) = reader.read(&mut buf).await {
                println!("{}", String::from_utf8_lossy(&buf));
            }
        });

        let mut stdin_buf_reader = BufReader::new(stdin());
        let mut buf = String::new();
        while let Ok(_) = stdin_buf_reader.read_line(&mut buf).await {
            writer.write_all(buf.as_bytes()).await.unwrap();
            buf.clear();
        }
    }
}
