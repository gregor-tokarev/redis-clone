use std::str;
use std::sync::Arc;
use std::time::Duration;

use command_router::Command;
use executor::execute_command;
use storage::Storage;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod command_router;
mod executor;
mod storage;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //

    let storage = Arc::new(Mutex::new(Storage::new()));

    let stor = storage.clone();
    tokio::spawn(async move {
        loop {
            // stor.lock().await.tick();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Port already in use");

    loop {
        let storage_clone = storage.clone();
        let stream = listener.accept().await;

        match stream {
            Ok((mut s, _)) => {
                tokio::spawn(async move {
                    loop {
                        let mut buf = [0u8; 512];

                        s.read(&mut buf).await.expect("Error reading buffer");

                        let command_str = str::from_utf8(&buf).unwrap();
                        if let Ok(com) = Command::new(command_str) {
                            let mut storage_guard = storage_clone.lock().await;
                            execute_command(com, &mut s, &mut *storage_guard).await;
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
