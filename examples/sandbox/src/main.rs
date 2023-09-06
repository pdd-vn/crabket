use anyhow::Result;
use crabket_server::{Server, ServerMode};
use std::{net::Ipv4Addr, str::FromStr};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "192.168.1.100")]
    host: String,
    #[arg(long, default_value_t = 8686)]
    port: u16,
    #[arg(long, default_value = "single")]
    mode: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let host = args.host;
    let port = args.port;
    let mode: ServerMode;
    if args.mode == "single" {
        mode = ServerMode::SingleThread
    } else {
        mode = ServerMode::MultiThread
    }
    let server = Server::new(Ipv4Addr::from_str(&host)?, port, mode);
    let _ = server.run();
    Ok(())
}
