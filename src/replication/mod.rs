use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::resp_utils::{build_array, build_bulk};
use crate::{args::Args, http::Http};

#[derive(Debug)]
pub(crate) struct Replication {
    pub is_master: bool,
    pub master_id: Option<String>,
    pub master_host: Option<String>,
    pub master_port: Option<isize>,
    args: Args,
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
            println!("I'm master");
            return;
        };

        println!("I'm slave");
        println!("{:?}", self);

        if let (Some(master_host), Some(master_port)) = (&self.master_host, self.master_port) {
            let url = format!("{}:{}", master_host, master_port);

            let res1 = self.ping_master(&url).await;
            let res2 = self.ping_master_port(&url).await;
            let res3 = self.ping_master_capabilities(&url).await;

            println!("{:?}", vec![res1, res2, res3]);
        }
    }

    async fn ping_master(&self, url: &str) -> String {
        let body = build_array(vec![build_bulk("PING".to_owned())]);

        Http::new(url, body).make_request().await
    }

    async fn ping_master_capabilities(&self, url: &str) -> String {
        let body = build_array(vec![
            build_bulk("REPLCONF".to_owned()),
            build_bulk("capa".to_owned()),
            build_bulk("psync2".to_owned()),
        ]);

        Http::new(url, body).make_request().await
    }
    async fn ping_master_port(&self, url: &str) -> String {
        let body = build_array(vec![
            build_bulk("REPLCONF".to_owned()),
            build_bulk("listening-port".to_owned()),
            build_bulk(format!("{}", self.args.port)),
        ]);

        Http::new(url, body).make_request().await
    }
}
