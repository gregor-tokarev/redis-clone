use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::Command;

pub async fn execute_command(command: Command, soket: &mut TcpStream) {
   match command {
       Command::Ping => soket.write_all("+PONG\r\n".as_bytes()).await.unwrap(),
       Command::Echo(echo_stirng) => soket.write_all(format!("+{}\r\n", echo_stirng).as_bytes()).await.unwrap(),
       Command::Unrecognized => println!("Unrecognized command")
   };
}
