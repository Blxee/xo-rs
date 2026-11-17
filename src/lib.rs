mod xo;
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

pub enum Protocol {}
