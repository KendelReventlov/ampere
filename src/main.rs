use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;

fn conectar(stream: &mut TcpStream){

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    println!("MENSAJE RECIVIDO: {:?}",String::from_utf8(buffer).unwrap());
}

fn main() {
    let servidor = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in servidor.incoming(){
        conectar(&mut stream.unwrap());
    }
}
