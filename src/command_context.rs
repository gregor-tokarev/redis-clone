use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

use crate::{args::Args, replication::Replication, storage::Storage};

pub(crate) struct CommandContext {
    pub replication_info: Mutex<Replication>,
    pub storage: Mutex<Storage>,
    pub args: Args,
    pub blocking_tx: Mutex<Sender<()>>,
    pub blocing_rx: Mutex<Receiver<()>>,
    pub in_block: Mutex<bool>,
}

impl CommandContext {
    pub fn new(replication: Replication, storage: Storage, args: Args) -> Self {
        let (tx, rx) = channel(1);

        Self {
            replication_info: Mutex::new(replication),
            storage: Mutex::new(storage),
            args,
            blocing_rx: Mutex::new(rx),
            blocking_tx: Mutex::new(tx),
            in_block: Mutex::new(false),
        }
    }
}
