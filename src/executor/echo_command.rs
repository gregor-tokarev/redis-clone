use crate::{
    command_router::{Command, EchoCommand},
    storage::Storage,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn echo_command(socket: &mut TcpStream, _storage: &mut Storage, command: EchoCommand) {
   socket
            .write_all(format!("+{}\r\n", command.echo).as_bytes())
            .await
            .unwrap();
}
