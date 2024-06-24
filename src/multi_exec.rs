use std::future::Future;
use std::pin::Pin;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub(crate) struct MultiexecContainer {
    pub active: bool,
    pub execution_queue: Vec<Box<()>>,
}

impl MultiexecContainer {
    pub(crate) fn new() -> Self {
        Self {
            execution_queue: vec![],
            active: false
        }
    }

    pub(crate) async fn store_action(&mut self, action: Box<()>) {
        self.execution_queue.push(action)
    }
}
