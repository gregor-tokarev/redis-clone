use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use item::{Item, StreamDataEntry};
use tokio::sync::Mutex;

use crate::command_router::{XRangeCommand, XRangeStatement};

pub mod item;

pub type StorageState = HashMap<String, Item>;

pub type StorageExpire = HashMap<String, u128>;

pub struct Storage {
    state: Arc<Mutex<StorageState>>,
    expire_list: Arc<Mutex<StorageExpire>>,
}
impl Storage {
    pub fn new(dump: (StorageState, StorageExpire)) -> Self {
        Self {
            state: Arc::new(Mutex::new(dump.0)),
            expire_list: Arc::new(Mutex::new(dump.1)),
        }
    }

    pub async fn set(&mut self, key: String, value: Item, expire_after: Option<Duration>) {
        self.state.lock().await.insert(key.clone(), value);

        if let Some(expr_after) = expire_after {
            let now = self.now();
            let expire_at = now + expr_after;

            self.expire_list
                .lock()
                .await
                .insert(key, expire_at.as_millis());
        }
    }

    pub async fn xadd(&mut self, key: String, id: String, data: HashMap<String, String>) {
        let item = self.get(key.as_str()).await;
        let mut state = self.state.lock().await;

        match item {
            Some(itm) => {
                if let Item::Stream(mut stream) = itm {
                    stream.value.push(item::StreamDataEntry {
                        id: id.clone(),
                        data,
                    });

                    state.insert(key, Item::Stream(stream));
                };
            }
            None => {
                state.insert(
                    key,
                    Item::Stream(item::StreamData {
                        value: vec![StreamDataEntry { id, data }],
                    }),
                );
            }
        };
    }

    pub async fn xrange(&self, xrange: XRangeCommand) -> Option<Vec<Vec<StreamDataEntry>>> {
        let item = self.get(xrange.key.as_str()).await;

        if let Some(Item::Stream(stream)) = item {
            let mut all_key_items = vec![];

            let mut same_timestamp: Vec<StreamDataEntry> = vec![];
            for entry in stream.value {
                let (timestamp, count) = entry.split_id().unwrap();

                match xrange.start_statement {
                    XRangeStatement::Id(start_id) => {
                        if let (Some(start_timestamp), Some(start_count)) = start_id {
                            match xrange.end_statement {
                                XRangeStatement::Id(end_id) => {
                                    if let (Some(end_timestamp), Some(end_count)) = end_id {
                                        println!("Comparison:");
                                        println!(
                                            "Timestamp - {} <= {} <= {}",
                                            start_timestamp, timestamp, end_timestamp
                                        );
                                        println!(
                                            "Count - {} <= {} <= {}\n",
                                            start_count, count, end_count
                                        );
                                        if (start_timestamp <= timestamp
                                            && timestamp <= end_timestamp)
                                            && (start_count <= count && count <= end_count)
                                        {
                                            println!("Entry - {:?}", entry);
                                            match same_timestamp.clone().last() {
                                                Some(e) => {
                                                    println!("{:?}", e.clone());
                                                    let (last_temp_entry_timestamp, _) =
                                                        e.split_id().unwrap();

                                                    if last_temp_entry_timestamp == timestamp {
                                                        same_timestamp.push(entry)
                                                    } else if last_temp_entry_timestamp < timestamp
                                                    {
                                                        all_key_items.push(same_timestamp.clone());
                                                        same_timestamp.clear();
                                                    }
                                                }
                                                None => same_timestamp.push(entry),
                                            };

                                            println!("{:?}", same_timestamp.clone());
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                };
                println!("\n\n\n");
            }

            if !same_timestamp.is_empty() {
               all_key_items.push(same_timestamp);
            }

            return Some(all_key_items);
        };

        None
    }

    pub async fn get_top_stream_item(&self, key: String) -> Option<StreamDataEntry> {
        let item = self.get(key.as_str()).await;

        match item {
            Some(itm) => {
                if let Item::Stream(stream) = itm {
                    let entry = stream.clone().value.last().unwrap().clone();
                    Some(entry)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn now(&self) -> Duration {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }

    pub async fn remove(&self, key: &str) {
        self.state.lock().await.remove(key);
    }

    pub async fn get(&self, key: &str) -> Option<Item> {
        let mut state = self.state.lock().await;
        let mut expire_list = self.expire_list.lock().await;

        if let Some(expire_time) = expire_list.get(key) {
            if self.now().as_millis() >= *expire_time {
                state.remove(key);
                expire_list.remove(key);
                return None;
            }
        };

        state.get(key).cloned()
    }

    pub async fn keys(&self, _pattern: String) -> Vec<String> {
        let state = self.state.lock().await;

        state.keys().cloned().collect()
    }

    pub async fn tick(&mut self) {
        let current_time = self.now().as_millis();
        let mut keys = vec![];

        {
            let expire_list = self.expire_list.lock().await;
            for (key, value) in expire_list.iter() {
                if current_time >= *value {
                    keys.push(key.clone());
                }
            }
        }

        if !keys.is_empty() {
            let mut expire_list = self.expire_list.lock().await;
            for key in &keys {
                expire_list.remove(key);
            }
        };

        for key in keys {
            self.remove(&key).await;
        }
    }
}
