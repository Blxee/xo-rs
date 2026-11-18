mod xo;
use core::fmt;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
pub use xo::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(default_value = "127.0.0.1")]
    pub host: String,
    #[arg(default_value = "8888")]
    pub port: u16,
}

impl ToString for Args {
    fn to_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Protocol {
    pub game: Option<XOGame>,
    pub state: State,
}

#[derive(Serialize, Deserialize)]
pub enum State {
    WaitingForPlayer,
    Started,
    YourTurn,
    Move(Move),
    End,
}

impl Protocol {
    pub async fn send<T: AsyncWrite + Unpin>(&self, writer: &mut BufWriter<T>) {
        let bytes = postcard::to_stdvec(self).unwrap();
        writer.write_u32(bytes.len() as u32).await.unwrap();
        writer.write_all(&bytes).await.unwrap();
        writer.flush().await.unwrap();
    }

    pub async fn recv<T: AsyncRead + Unpin>(reader: &mut BufReader<T>) -> Result<Self, ()> {
        let size = reader.read_u32().await.unwrap() as usize;
        let mut buf = vec![0; size];
        reader.read_exact(&mut buf).await.unwrap();
        Ok(postcard::from_bytes(&buf).unwrap())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::WaitingForPlayer => write!(f, "waiting for opponent to make a move.."),
            State::Started => write!(f, "game started"),
            State::End => write!(f, "game ended"),
            State::Move(_) => todo!(),
            State::YourTurn => todo!(),
        }
    }
}
