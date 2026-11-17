use std::{
    error::Error,
    io::{Read, Write, stdout},
};

use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, stdin},
    join,
    net::TcpStream,
    spawn,
};
use xo::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = Args::parse().to_string();
    let socket = TcpStream::connect(addr).await?;

    let (reader, mut writer) = socket.into_split();

    let handle = spawn(async move {
        let mut buf = String::new();
        loop {
            print!("[in]: ");
            stdout().flush().unwrap();
            buf.push_str("hello world");
            writer.write_all(buf.as_bytes()).await.unwrap();
            writer.flush().await.unwrap();
            buf.clear();
        }
    });

    let _ = join!(handle);
    Ok(())
}
