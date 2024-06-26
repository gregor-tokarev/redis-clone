use args::Args;
use clap::Parser;
use command_context::CommandContext;
use command_router::Command;
use executor::execute_command;
use rdb::RDB;
use replication::Replication;
use std::sync::Arc;
use storage::Storage;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use transaction::TransactionContainer;

mod args;
mod command_context;
mod command_router;
mod executor;
mod rdb;
mod replication;
mod resp_utils;
mod storage;
mod tcp_request;
mod transaction;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut rdb = RDB::new(args.clone());
    let dump = rdb.load_dump().await.unwrap();

    let context = Arc::new(CommandContext::new(
        Replication::new(args.clone()),
        Storage::new(dump),
        args.clone(),
    ))
    .clone();

    // Async agents
    let context_clone_expire = context.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            context_clone_expire.storage.lock().await.tick().await;
        }
    });
    // End of async agents

    context.replication_info.lock().await.connect_master().await;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .expect("Port already in use");

    initial_greeting(args.clone());

    loop {
        let context_clone = context.clone();
        let stream = listener.accept().await;

        match stream {
            Ok((mut s, _)) => {
                tokio::spawn(async move {
                    let transaction_container = Arc::new(Mutex::new(TransactionContainer::new()));

                    loop {
                        let mut buffer = [0u8; 1024];
                        let mut command_str = String::new();

                        let bytes_read = s.read(&mut buffer).await.unwrap();

                        command_str = format!(
                            "{}{}",
                            command_str,
                            String::from_utf8_lossy(&buffer[..bytes_read])
                        );

                        let tx = transaction_container.clone();
                        if let Ok(com) = Command::new(&command_str) {
                            execute_command(com, &mut s, &context_clone, tx).await;
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

fn initial_greeting(args: Args) {
    println!(
        r###"
                _._
           _.-``__ ''-._
      _.-``    `.  `_.  ''-._           
  .-`` .-```.  ```\/    _.,_ ''-._
 (    '      ,       .-`  | `,    )    Hi there!
 |`-._`-...-` __...-.``-._|'` _.-'|    I implement redis  
 |    `-._   `._    /     _.-'    |    My github: https://github.com/gregor-tokarev 
  `-._    `-._  `-./  _.-'    _.-'     
 |`-._`-._    `-.__.-'    _.-'_.-'|    Port: {}
 |    `-._`-._        _.-'_.-'    | 
  `-._    `-._`-.__.-'_.-'    _.-'
 |`-._`-._    `-.__.-'    _.-'_.-'|
 |    `-._`-._        _.-'_.-'    |
  `-._    `-._`-.__.-'_.-'    _.-'
      `-._    `-.__.-'    _.-'
          `-._        _.-'
              `-.__.-'
             "###,
        args.port
    );
}
