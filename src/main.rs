
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/// Write text to a stream. (HTTP)
fn write_txt(txt: &str, mut stream: &TcpStream) -> Result<usize, io::Error>{
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                           txt.len(), txt);
    let written = stream.write(response.as_bytes());
    stream.flush().unwrap();
    written
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();
    if res.is_partial() {
        match req.path {
            Some(ref path) => {
                println!("Path: {}", path);
            },
            None => {
                println!("No path");
            }
        }
    }
    println!("Req: {:?}", req);

    let contents = "Hello from rust";
    let _ = write_txt(contents, &stream);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;
    println!("Listening on port 8080");
    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connection established!");
        handle_connection(stream);
    }
    Ok(())
}