use std::str;

use command_router::Command;
use executor::execute_command;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

mod command_router;
mod executor;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //

    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Port already in use");

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut s, _)) => {
                tokio::spawn(async move {
                    loop {
                        let mut buf = [0u8; 512];

                        s.read(&mut buf).await.expect("Error reading buffer");

                        let command_str = str::from_utf8(&buf).unwrap();
                        if let Ok(com) = Command::new(command_str){
                            execute_command(com, &mut s).await;
                        }
                    }
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
}

