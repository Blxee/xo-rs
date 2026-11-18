use std::{collections::VecDeque, error::Error, net::SocketAddr, sync::Arc};

use clap::Parser;
use tokio::{
    io::{AsyncReadExt, BufReader, BufWriter},
    join,
    net::{TcpListener, TcpStream},
    spawn,
    sync::{Mutex, Notify},
};
use xo::{Args, GameState, Protocol, State, Symbol, XOGame};

struct Player {
    socket: TcpStream,
    addr: SocketAddr,
    sym: Symbol,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = Args::parse().to_string();

    let listener = TcpListener::bind(addr.clone()).await?;
    println!("listening at {addr}..");

    let mut waiting_list = VecDeque::new();

    let mut handles = Vec::new();

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("client connected from {addr}");

                let mut player1 = Player {
                    socket,
                    addr,
                    sym: Symbol::O,
                };

                if let Some(player2) = waiting_list.pop_front() {
                    player1.sym = Symbol::X;
                    let handle = spawn(async move {
                        start_game(player1, player2).await;
                    });
                    handles.push(handle);
                } else {
                    waiting_list.push_front(player1);
                }
            }
            Err(_) => eprintln!("could not accept client connetion"),
        }
    }
}

async fn start_game(mut player1: Player, mut player2: Player) {
    println!("{} and {} are playing..", player1.addr, player2.addr);

    // let game = Arc::new((Notify::new(), Mutex::new(XOGame::new())));
    let game = XOGame::new();
    if player1.sym != game.get_turn() {
        (player1, player2) = (player2, player1);
    }

    let (reader, writer) = player1.socket.into_split();
    let (mut p1_reader, mut p1_writer) = (BufReader::new(reader), BufWriter::new(writer));

    let (reader, writer) = player2.socket.into_split();
    let (mut p2_reader, mut p2_writer) = (BufReader::new(reader), BufWriter::new(writer));

    let start_msg = Protocol {
        game: Some(game.clone()),
        state: State::Started,
    };

    start_msg.
}
