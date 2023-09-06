use anyhow::Result;
use log::{error, info, warn};
use std::io::Read;
use std::io::Write;
use std::net::Ipv4Addr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

const BUF_SIZE: usize = 1024;
const HTTP_METHODS: [&str; 9] = [
    "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE",
];

#[derive(Debug)]
pub enum ServerMode {
    MultiThread,
    SingleThread,
}

pub struct Server {
    ip: Ipv4Addr,
    port: u16,
    mode: ServerMode,
}

fn is_http_request(method: &str) -> bool {
    let tokens: Vec<&str> = method.split(" ").collect();
    if HTTP_METHODS.contains(&tokens[0]) && tokens[tokens.len() - 1].starts_with("HTTP") {
        true
    } else {
        false
    }
}

fn handle_tcpstream(stream: &mut TcpStream) {
    let src_addr = stream.peer_addr().expect("Failed to get peer address");
    info!("Got connection from: {:?}", src_addr);

    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let nbytes = stream.read(&mut buf).expect("Failed to read stream");
    let msg = String::from_utf8_lossy(&buf[..nbytes]);

    info!("Received {} bytes", nbytes);

    let tokens: Vec<&str> = msg.split("\r\n").collect();

    // Currently only handle packet with smaller size than buffer
    match tokens.clone().into_iter().position(|x| x == "") {
        Some(idx) => {
            let method = tokens[0];
            let headers = &tokens[1..idx + 1];
            let body = tokens[idx + 1];

            let mut response = String::from("OK");
            if is_http_request(method) {
                info!("Method: {}", method);
                info!("Headers: {:?}", headers);
                info!("Body: {}", body);

                response = String::from("HTTP/1.1 200 OK");
                if body == "ping" {
                    response.push_str("\r\n\r\npong\n")
                }
            }

            let _ = stream.write(response.as_bytes());
        }
        None => {
            warn!("Malicious packet format: {msg}")
        }
    }
}

impl Server {
    pub fn new(ip: Ipv4Addr, port: u16, mode: ServerMode) -> Self {
        Server { ip, port, mode }
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        env_logger::init();
        let addr = format!("{}:{}", self.ip, self.port);
        let listener = TcpListener::bind(addr)?;

        info!("Starting server:");
        info!("- Host: {}", self.ip);
        info!("- Port: {}", self.port);
        info!("- Mode: {:?}", self.mode);

        for stream_result in listener.incoming() {
            info!("{}", "+".repeat(60));
            match stream_result {
                Ok(mut stream) => match self.mode {
                    ServerMode::MultiThread => {
                        thread::spawn(move || handle_tcpstream(&mut stream));
                    }
                    ServerMode::SingleThread => handle_tcpstream(&mut stream),
                },
                Err(e) => {
                    error!("Failed to receive stream: {:?}", e)
                }
            }
        }

        Ok(())
    }
}
