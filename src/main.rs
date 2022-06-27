
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/// Write text to a stream. (HTTP)
/// Pretty naive
/// # Arguments
/// * `stream` - The stream to write to.
/// * `text` - The text to write.
/// * `status` - The status code to write. (It's a string lol)
/// # Returns
/// * `Ok(usize)` - The number of bytes written.
/// * `Err(io::Error)` - The error.
fn write_txt(txt: &str, stream: &mut TcpStream, status: &str) -> Result<usize, io::Error>{
    let response = format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\nServer: demo-rust-http-server/0.1.0\r\n\r\n{}",
        status, txt.len(), txt);
    let written = stream.write(response.as_bytes());
    stream.flush().unwrap(); // probably should not unwrap but whatever
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
    match req.path {
        Some(ref path) => {
            if path == &"/" {
                let txt = std::fs::read_to_string("pages/index.html").unwrap();
                let _ = write_txt(&txt, &mut stream, "200 OK");
            } else {
                let txt = std::fs::read_to_string("pages/404.html").unwrap();
                let _ = write_txt(&txt, &mut stream, "404 Not Found");
            }
        },
        None => {
            let txt = "No path";
            let _ = write_txt(txt, &mut stream, "400 Bad Request");
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;
    println!("Listening on port 8080");
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream);
    }
    Ok(())
}