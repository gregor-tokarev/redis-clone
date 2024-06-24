use std::{
    char,
    collections::HashMap,
    time::{Duration},
    u64,
};


use tokio::{
    fs,
    io::{AsyncReadExt, BufReader},
};

use crate::{
    args::Args,
    storage::{Item, StorageExpire, StorageState},
};

pub struct RDB {
    dirname: String,
    dbfilename: String,
    file: Option<fs::File>,
}

impl RDB {
    pub fn new(args: Args) -> Self {
        Self {
            dirname: args.dir,
            dbfilename: args.dbfilename,
            file: None,
        }
    }

    async fn ensure_file_exists(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.file_path();

        match fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path.clone())
            .await
        {
            Ok(file) => {
                self.file = Some(file);
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                // File exists, try opening it
                match fs::File::open(path).await {
                    Ok(file) => {
                        self.file = Some(file);
                        Ok(())
                    }
                    Err(_) => Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to open existing file",
                    ))),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn file_path(&self) -> String {
        format!("{}/{}", self.dirname, self.dbfilename)
    }

    pub async fn start_sync(&mut self) -> Result<(StorageState, StorageExpire), Box<dyn std::error::Error>> {
        if self.file.is_none() {
            self.ensure_file_exists().await.unwrap();
        };

        if let Some(ref mut file) = self.file {
            println!("Found dump file, loading...");
            let mut buffer = Vec::new();

            let mut reader = BufReader::new(file);
            reader.read_to_end(&mut buffer).await?;

            if buffer.is_empty() {
                return Ok((HashMap::new(), HashMap::new()));
            };

            let mut iter = buffer.into_iter().skip_while(|&b| b != 0xfb).skip(1);

            let mut loaded_state: StorageState = HashMap::new();
            let mut loaded_expiries: StorageExpire = HashMap::new();

            let hashtable_size = iter.next().unwrap();
            let _expire_hashtable_size = iter.next().unwrap();

            for _ in 0..hashtable_size {
                let value_header = iter.next().unwrap();

                // Expire in milseconds
                let expire = if value_header == 0xfc {
                    let mut expire = [0u8; 8];
                    for i in 0..8 {
                        expire[i] = iter.next().unwrap()
                    }

                    Some(u64::from_le_bytes(expire))
                } else if value_header == 0xfd {
                    let mut expire = [0u8; 8];
                    for i in 0..4 {
                        expire[i] = iter.next().unwrap()
                    }

                    Some(u64::from_le_bytes(expire))
                } else {
                    None
                };

                if expire.is_some() {
                    let _value_type = iter.next().unwrap();
                }

                let key_len = iter.next().unwrap();
                let mut key_buf: Vec<char> = vec![];
                for _j in 0..key_len {
                    let c = iter.next().unwrap();
                    key_buf.push(c as char)
                }
                let key = key_buf.into_iter().collect::<String>();

                if let Some(exp) = expire {
                    let duration = if value_header == 0xfc {
                        Duration::from_millis(exp)
                    } else {
                        Duration::from_secs(exp)
                    };

                    loaded_expiries.insert(key.clone(), duration.as_millis());
                }

                let value_len = iter.next().unwrap();
                let mut value_buf: Vec<char> = vec![];
                for _j in 0..value_len {
                    let c = iter.next().unwrap();
                    value_buf.push(c as char);
                }

                loaded_state.insert(key, Item::SimpleString(value_buf.into_iter().collect()));
            }

            println!("Dump loaded.");

            Ok((loaded_state, loaded_expiries))
        } else {
            Ok((HashMap::new(), HashMap::new()))
        }

        // println!("wanna write");
        // if let Some(file) = &mut self.file {
        //     println!("{:?}", bincode::serialize("REDIS0007")?);
        //     println!("{:?}", b"REDIS0007");
        //     file.write_all(&[52, 45, 44, 49, 53, 30, 30, 30, 37])
        //         .await
        //         .unwrap();
        //
        //     Ok(())
        // } else {
        //     Err(Box::new(std::io::Error::new(
        //         std::io::ErrorKind::Other,
        //         "File not available",
        //     )))
        // }
    }
}
