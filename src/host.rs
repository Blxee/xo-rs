use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, stdin},
    net::{TcpListener, UdpSocket},
};

use crate::{CONNECT_PORT, SEARCH_PORT};

pub(crate) async fn host_main() {
    let mut search_port = SEARCH_PORT;
    let search_socket = loop {
        println!("trying to bind to udp port {search_port}..");

        match UdpSocket::bind(("0.0.0.0", search_port)).await {
            Ok(socket) => break socket,
            Err(_) => (),
        }
        search_port += 1;
    };

    let mut connect_port = CONNECT_PORT;
    let listener = loop {
        println!("trying to listen to tcp port {connect_port}..");

        match TcpListener::bind(("0.0.0.0", connect_port)).await {
            Ok(listener) => break listener,
            Err(_) => (),
        }
        connect_port += 1;
    };
    let connect_port_msg = connect_port.to_string();

    let mut buf = [0u8; 4];
    let (client_socket, client_addr) = loop {
        match search_socket.recv_from(&mut buf).await {
            Ok((_, client_addr)) if buf == *b"PING" => {
                search_socket
                    .send_to(connect_port_msg.as_bytes(), client_addr)
                    .await
                    .unwrap();
                break listener.accept().await.unwrap();
            }
            _ => continue,
        }
    };

    drop(search_socket);

    println!("client {} connected..", client_addr);

    let (mut reader, mut writer) = client_socket.into_split();

    tokio::spawn(async move {
        let mut buf = [0u8; 1024];

        while let Ok(1..) = reader.read(&mut buf).await {
            println!("{}", String::from_utf8_lossy(&buf).trim());
        }
    });

    let mut stdin_buf_reader = BufReader::new(stdin());
    let mut buf = String::new();
    while let Ok(_) = stdin_buf_reader.read_line(&mut buf).await {
        writer.write_all(buf.as_bytes()).await.unwrap();
        buf.clear();
    }
}
