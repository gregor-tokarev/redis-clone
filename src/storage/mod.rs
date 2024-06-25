use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use item::{Item, StreamDataEntry};
use tokio::sync::Mutex;

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
