use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
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