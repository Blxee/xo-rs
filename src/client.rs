use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, stdin},
    net::TcpStream,
};

pub(crate) async fn client_main(host_socket: TcpStream) {
    println!("connected to host");

    let (mut reader, mut writer) = host_socket.into_split();

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

#[derive(Serialize, Deserialize)]
enum ClientProtocol {
    Join,
    Leave,
}
