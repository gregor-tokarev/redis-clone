use core::str;
use std::{char, isize};

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::resp_utils::{build_array, build_bulk};


pub(crate) struct Replication {
    pub is_master: bool,
    pub master_id: Option<String>,
    pub master_host: Option<String>,
    pub master_port: Option<isize>
}

impl Replication {
    pub fn new(master_connection_string: Option<String>) -> Self {
       if let Some(conn) = master_connection_string {
           let resp = conn.split(' ').collect::<Vec<&str>>();

           Self {
               is_master: false,
               master_id: None,
               master_host: Some(resp[0].to_string()),
               master_port: Some(resp[1].parse().unwrap())
           }
       } else {
           Self {
               is_master: true,
               master_id: Some(Self::generate_master_id()),
               master_host: None,
               master_port: None
           }
       }
    }

    fn generate_master_id() -> String {
        thread_rng().sample_iter(Alphanumeric).take(40).map(char::from).collect()
    }

    pub async fn ping_master(&self) {
        if self.is_master {
            return
        };

        if let (Some(master_host), Some(master_port)) = (&self.master_host, self.master_port) {
            let mut stream = TcpStream::connect(format!("{}:{}", master_host, master_port)).await.unwrap();

            stream.write_all(build_array(build_bulk("PING".to_owned())).as_bytes()).await.unwrap();
        }
    }
}
