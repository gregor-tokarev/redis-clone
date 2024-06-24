use args::Args;
use clap::Parser;
use command_context::CommandContext;
use command_router::Command;
use executor::execute_command;
use rdb::RDB;
use replication::Replication;
use multi_exec::MultiexecContainer;
use std::sync::Arc;
use storage::Storage;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod args;
mod command_context;
mod command_router;
mod executor;
mod rdb;
mod replication;
mod resp_utils;
mod storage;
mod tcp_request;
mod multi_exec;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut rdb = RDB::new(args.clone());
    let dump = rdb.start_sync().await.unwrap();

    let transaction_container = MultiexecContainer::new();

    let context = Arc::new(CommandContext::new(
        Replication::new(args.clone()),
        Storage::new(dump),
        transaction_container,
        args.clone(),
    ));

    context.replication_info.lock().await.connect_master().await;

    let stor = context.clone();
    tokio::spawn(async move {
        loop {
            stor.storage.lock().await.tick().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

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
                    let mut buffer = [0u8; 1024];
                    while let Ok(bytes_read) = s.read(&mut buffer).await {
                        if bytes_read == 0 {
                            break;
                        }

                        let command_str =
                            String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

                        if let Ok(com) = Command::new(&command_str) {
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

fn initial_greeting(args: Args) {
    println!(r###"
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
             "###, args.port);
}
