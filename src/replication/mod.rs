use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::resp_utils::{build_array, build_bulk};
use crate::{args::Args, tcp_request::TcpRequest};

#[derive(Debug)]
pub(crate) struct Replication {
    pub is_master: bool,
    pub master_id: Option<String>,
    pub master_host: Option<String>,
    pub master_port: Option<isize>,
    pub args: Args
}

impl<'a> Replication {
    pub fn new(args: Args) -> Self {
        if let Some(conn) = args.replicaof.clone() {
            let resp = conn.split(' ').collect::<Vec<&str>>();

            Self {
                args,
                is_master: false,
                master_id: None,
                master_host: Some(resp[0].to_string()),
                master_port: Some(resp[1].parse().unwrap()),
            }
        } else {
            Self {
                args,
                is_master: true,
                master_id: Some(Self::generate_master_id()),
                master_host: None,
                master_port: None,
            }
        }
    }

    fn generate_master_id() -> String {
        thread_rng()
            .sample_iter(Alphanumeric)
            .take(40)
            .map(char::from)
            .collect()
    }

    pub async fn connect_master(&self) {
        if self.is_master {
            return;
        };

        if let (Some(master_host), Some(master_port)) = (&self.master_host, self.master_port) {
            let url = format!("{}:{}", master_host, master_port);

            self.ping_master(&url).await;
            self.ping_master_port(&url).await;
            self.ping_master_capabilities(&url).await;
        }
    }

    async fn ping_master(&self, url: &str) -> String {
        let body = build_array(vec![build_bulk("PING".to_owned())]);

        TcpRequest::new(url, body).make_request().await
    }

    async fn ping_master_capabilities(&self, url: &str) -> String {
        let body = build_array(vec![
            build_bulk("REPLCONF".to_owned()),
            build_bulk("capa".to_owned()),
            build_bulk("psync2".to_owned()),
        ]);

        TcpRequest::new(url, body).make_request().await
    }
    async fn ping_master_port(&self, url: &str) -> String {
        let body = build_array(vec![
            build_bulk("REPLCONF".to_owned()),
            build_bulk("listening-port".to_owned()),
            build_bulk(format!("{}", self.args.port)),
        ]);

        TcpRequest::new(url, body).make_request().await
    }
}
