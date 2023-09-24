use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::error::Error;

struct Request {
    pub stream: TcpStream,
    pub buffer: [u8; 1024],
}

impl Request {
    pub fn new(stream: TcpStream) -> Self {
        Request {
            stream,
            buffer: [0; 1024]
        }
    }

    pub fn read(&mut self) -> String {
        self.stream.read(&mut self.buffer).unwrap();
        String::from_utf8_lossy(&self.buffer).to_string()
    }

    pub fn reply(&mut self, response: &str) -> Result<(), Box<dyn Error>> {
        let byte_response = response.as_bytes();
        match self.stream.write(byte_response) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err) as Box<dyn Error>)
        }
    }

    pub fn shutdown(self) -> Result<(), Box<dyn Error>> {
        match self.stream.shutdown(Shutdown::Both) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err) as Box<dyn Error>)
        }
    }

}

fn handle_request(mut request: Request) -> Result<(), Box<dyn Error>> {
    let _string_request = request.read();
    // Format request
    let response = "HTTP/1.1 200 OK\r\n\r\nEverything worked as expected";

    let _ = request.reply(response);

    let _ = request.shutdown();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let request = Request::new(stream);
        handle_request(request)?;
    }
    Ok(())
}
