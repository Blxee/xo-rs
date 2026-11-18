use std::{
    error::Error,
    io::{Read, Write},
};

use clap::Parser;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter, stdin, stdout},
    join,
    net::TcpStream,
    spawn,
};
use xo::{Args, Move, Protocol, State};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = Args::parse().to_string();
    let socket = TcpStream::connect(addr).await?;

    let (reader, writer) = socket.into_split();
    let (mut reader, mut writer) = (BufReader::new(reader), BufWriter::new(writer));

    let handle = spawn(async move { loop {} });

    let mut input = BufReader::new(stdin());
    let mut buf = String::new();
    loop {
        let Ok(msg) = Protocol::recv(&mut reader).await else {
            eprintln!("server disconnected..");
            break;
        };

        if let Some(game) = msg.game {
            println!("{game}");
        }

        match msg.state {
            State::YourTurn => {
                let user_move = loop {
                    print!("[in]: ");
                    stdout().flush().await.unwrap();
                    buf.clear();
                    input.read_line(&mut buf).await.unwrap();
                    if let Ok(user_move) = buf.as_str().try_into() {
                        break user_move;
                    }
                    buf.clear();
                };
                let msg = Protocol {
                    game: None,
                    state: State::Move(user_move),
                };
                msg.send(&mut writer).await;
                writer.flush().await.unwrap();
            }
            State::End => {
                println!("{}", msg.state);
                break;
            }
            _ => println!("{}", msg.state),
        }
    }
    Ok(())
}
