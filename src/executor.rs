use std::time::Duration;

use crate::storage::{Item, Storage};
use crate::Command;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn execute_command(command: Command, socket: &mut TcpStream, storage: &mut Storage) {
    match command {
        Command::Ping => socket.write_all("+PONG\r\n".as_bytes()).await.unwrap(),
        Command::Echo(echo_stirng) => socket
            .write_all(format!("+{}\r\n", echo_stirng).as_bytes())
            .await
            .unwrap(),
        Command::Set(set_command) => {
            let duration = if let Some(expire) = set_command.expire_after {
                Some(Duration::from_millis(expire.parse().unwrap()))
            } else {
                None
            };

            storage.set(
                set_command.key.to_owned(),
                Item::SimpleString(set_command.value.to_owned()),
                duration,
            );

            socket.write_all(b"+OK\r\n").await.unwrap();
        }
        Command::Get(key) => {
            let value = storage.get(&key);

            match value {
                Some(v) => {
                    let resp = v.build_response_string();

                    socket.write_all(resp.as_bytes()).await.unwrap();
                }
                None => {
                    socket.write_all(b"$-1\r\n").await.unwrap();
                }
            };
        }
        Command::Unrecognized => println!("Unrecognized command"),
    };
}
