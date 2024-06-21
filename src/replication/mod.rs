pub(crate) struct Replication {
    pub is_master: bool,
    pub master_host: Option<String>,
    pub master_port: Option<isize>
}

impl Replication {
    pub fn new(master_connection_string: Option<String>) -> Self {
       if let Some(conn) = master_connection_string {
           let resp = conn.split(' ').collect::<Vec<&str>>();

           Self {
               is_master: false,
               master_host: Some(resp[0].to_string()),
               master_port: Some(resp[1].parse().unwrap())
           }
       } else {
           Self {
               is_master: true,
               master_host: None,
               master_port: None
           }
       }
    }
}
