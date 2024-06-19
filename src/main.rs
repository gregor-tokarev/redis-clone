// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::TcpListener, str};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Port already in use");
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buff = [0u8; 512];
                stream.read(&mut buff).unwrap();

                for _line in str::from_utf8(&buff).unwrap().lines() {
                    stream.write_all("+PONG\r\n".as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
