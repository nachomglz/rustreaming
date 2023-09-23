use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_request(mut stream: TcpStream) {

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection stablished!");
    }
}
