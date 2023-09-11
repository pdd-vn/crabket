use crabket_server::{Server, ServerMode};
use log::info;
use std::io::{Read, Write};
use std::time::Duration;
use std::{
    net::TcpStream,
    sync::{Arc, RwLock},
    thread,
};

#[test]
#[should_panic]
fn test_invalid_ip() {
    let _ = Server::new("127.0.0.0.1", 8685, ServerMode::SingleThread);
}

#[test]
fn test_tcp_packet() {
    let host = "0.0.0.0";
    let port = 8686;
    let rw_lock = Arc::new(RwLock::new(true));
    let server_thread_lock = rw_lock.clone();
    let main_thread_lock = rw_lock.clone();
    let server = Server::new(host, port, ServerMode::SingleThread);

    // Start server in separated thread
    let server_handler = thread::spawn(move || server.run(server_thread_lock));

    // Testing
    send_http_request(&format!("127.0.0.1:{port}"), Duration::from_secs(2));
    send_plain_tcp_packet(&format!("127.0.0.1:{port}"), Duration::from_secs(2));

    // Shutdown server
    if let Ok(mut write_guard) = main_thread_lock.write() {
        *write_guard = false;
    }
    let _ = server_handler.join();
}

fn send_plain_tcp_packet(addr: &str, timeout: Duration) {
    let mut stream = TcpStream::connect(addr).unwrap();
    let msg = "Hello";
    let _ = stream.write(msg.as_bytes()).unwrap();
    let mut buf: [u8; 1024] = [0; 1024];

    thread::sleep(timeout);
    let nbytes = stream.read(&mut buf).expect("Failed to receive response");
    let response = String::from_utf8_lossy(&buf[..nbytes]);
    info!("Send {msg} - Receive {response}");
    assert_eq!(response, msg)
}

fn send_http_request(addr: &str, timeout: Duration) {
    let mut stream = TcpStream::connect(addr).unwrap();
    let msg = "GET /foo HTTP/1.1\r\nAccept: text/plain\r\nContent-Type: text/plain\r\n\r\nping";
    let _ = stream.write(msg.as_bytes()).unwrap();
    let mut buf: [u8; 1024] = [0; 1024];

    thread::sleep(timeout);
    let nbytes = stream.read(&mut buf).expect("Failed to receive response");
    let response = String::from_utf8_lossy(&buf[..nbytes]);
    let tokens = response.split("\r\n").collect::<Vec<&str>>();
    let body = tokens[tokens.len() - 1];
    info!("Send:\n{msg}");
    info!("Receive:\n{response}");
    assert_eq!(body, "pong")
}
