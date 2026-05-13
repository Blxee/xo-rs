use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tokio::net::TcpStream;

pub(crate) async fn client_main(host_socket: TcpStream) {
    let js = to_string(&ClientProtocol::Join);
}

#[derive(Serialize, Deserialize)]
enum ClientProtocol {
    Join,
    Leave,
}
