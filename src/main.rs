use std::env;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

fn handler(stream: &mut TcpStream) {
    let mut buf = vec![];
    match stream.read_to_end(&mut buf) {
        Ok(_) => {
            let words = std::str::from_utf8(&buf).unwrap();
            println!("{}", words)
            },
        Err(ref e) => panic!("IO failed: {}", e),
    }

}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port = &args[1];
    let addr = format!("127.0.0.1:{}", port);

    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        handler(&mut stream?);
    }
    Ok(())
}
