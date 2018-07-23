use std::env;
use std::io::{self, Read};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handler(mut stream: TcpStream) {
    let mut buf = vec![0; 512];

    stream.read(&mut buf).unwrap();

    let ls = String::from_utf8_lossy(&buf[..]);

    if ls.starts_with("GET") {
        let response = format!("HTTP/1.1 200 OK\r\n\r\n<html><body>hello</body></html>");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port = &args[1];
    let addr = format!("127.0.0.1:{}", port);

    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler(stream);
    }
    Ok(())
}
