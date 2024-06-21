
use std::str;
use std::sync::Arc;
use args::Args; use clap::Parser;
use command_context::CommandContext;
use command_router::Command;
use executor::execute_command;
use replication::Replication;
use storage::Storage;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;


mod args;
mod replication;
mod command_router;
mod executor;
mod resp_utils;
mod storage;
mod command_context;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // let storage = Arc::new(Mutex::new(Storage::new()));
    // let replication = Arc::new(Mutex::new(Replication::new(args.replicaof)));

    let context = Arc::new(CommandContext::new(Replication::new(args.replicaof), Storage::new()));

    context.replication_info.lock().await.ping_master().await;

    let stor = context.clone();
    tokio::spawn(async move {
        loop {
            stor.storage.lock().await.tick();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .expect("Port already in use");

    loop {
        let context_clone = context.clone();
        let stream = listener.accept().await;

        match stream {
            Ok((mut s, _)) => {
                tokio::spawn(async move {
                    loop {
                        let mut buf = [0u8; 512];

                        s.read(&mut buf).await.expect("Error reading buffer");

                        let command_str = str::from_utf8(&buf).unwrap();
                        if let Ok(com) = Command::new(command_str) {
                            execute_command(com, &mut s, &context_clone).await;
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
