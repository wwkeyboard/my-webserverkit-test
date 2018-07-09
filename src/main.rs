use std::env;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

fn handler(mut stream: TcpStream) {
    let mut buf = vec![0; 512];

    stream.read(&mut buf).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buf[..]));
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
