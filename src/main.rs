use std::io::Write;
use std::net::{TcpListener, TcpStream, Shutdown};

fn handle_request(_stream: &TcpStream) -> String {
    "HTTP/1.1 200 OK\r\n\r\nEverything worked as expected".to_owned()
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let response = handle_request(&stream);

        stream.write(response.as_bytes())?;

        // Close connection stream
        stream.shutdown(Shutdown::Both).expect("Failed to shutdown connection stream");
    }
    Ok(())
}
