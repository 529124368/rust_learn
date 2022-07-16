use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() {
    let linster = TcpListener::bind("127.0.0.1:8088").unwrap();
    for l in linster.incoming() {
        let stream = l.unwrap();
        handle_connect(stream);
    }
}

fn handle_connect(mut stream: TcpStream) {
    let mut buff = [0; 512];
    stream.read(&mut buff).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buff[..]))
}
