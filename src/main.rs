extern crate image;
extern crate websocket;
use std::net::{TcpListener};
use std::io::prelude::*;

fn main() {
    let servidor = TcpListener::bind("127.0.0.1:3000").unwrap();
    std::thread::spawn(move ||{
        let proceso = std::process::Command::new("python").arg("nucleo_optico.py").status();
        println!("{:?}",proceso);
    });
    for stream in servidor.incoming(){
        std::thread::spawn(move ||{
            let mut stream = stream.unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();

            let metadata = image::load_from_memory(&mut buffer).unwrap();
            println!("METADATA: {:?}",metadata.into_bytes().len());
        });
    }
}
