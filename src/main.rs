extern crate image;
extern crate websocket;
extern crate viuer;
use std::net::{TcpListener};
use std::io::prelude::*;
use viuer::{Config,print};

fn main() {
    let servidor = TcpListener::bind("127.0.0.1:3000").unwrap();
    std::thread::spawn(move ||{
        let proceso = std::process::Command::new("python").arg("nucleo_optico.py").status().unwrap();
        println!("PROCESO: {:?}",proceso);
    });
    for stream in servidor.incoming(){
        std::thread::spawn(move ||{
            let mut stream = stream.unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();

            let config = Config{
                width: Some(100),
                height: Some(30),
                x:0,
                y:0,
                ..Default::default()
            };

            let metadata = image::load_from_memory(&mut buffer).unwrap();
            print(&metadata, &config).unwrap();
        });
    }
}
