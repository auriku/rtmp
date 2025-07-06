mod basic_header;
mod chunk;
mod handshake;
mod message_header;
mod session;
mod utils;
mod message;
use std::{net::TcpListener, thread};

use session::Session;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:1935").unwrap();
    println!("RTMP server listening on port 1935");

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                thread::spawn(move || {
                    let mut session = Session::new(&mut s);
                    session.handle()
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}