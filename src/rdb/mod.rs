use std::{char, collections::HashMap, fmt::write, hash::Hash};

use bytes::buf;
use tokio::{
    fs,
    io::{self, AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWriteExt, BufReader},
};

use crate::{
    args::Args,
    storage::{Item, StorageState},
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

    pub async fn start_sync(&mut self) -> Result<StorageState, Box<dyn std::error::Error>> {
        if self.file.is_none() {
            self.ensure_file_exists().await.unwrap();
        };

        if let Some(ref mut file) = self.file {
            println!("Found dump file, loading...");
            let mut buffer = Vec::new();
            //
            let mut reader = BufReader::new(file);
            reader.read_to_end(&mut buffer).await?;

            if buffer.is_empty() {
                return Ok(HashMap::new());
            };

            let mut iter = buffer.into_iter().skip_while(|&b| b != 0xfb).skip(1);

            let mut loaded_state: StorageState = HashMap::new();

            let hashtable_size = iter.next().unwrap();
            let _expire_hashtable_size = iter.next().unwrap();

            for _ in 0..hashtable_size {
                let _value_type = iter.next().unwrap();

                let key_len = iter.next().unwrap();
                let mut key_buf: Vec<char> = vec![];
                for _j in 0..key_len {
                    let c = iter.next().unwrap();
                    key_buf.push(c as char)
                }

                let value_len = iter.next().unwrap();
                let mut value_buf: Vec<char> = vec![];
                for _j in 0..value_len {
                    let c = iter.next().unwrap();
                    value_buf.push(c as char);
                }

                loaded_state.insert(
                    key_buf.into_iter().collect(),
                    Item::SimpleString(value_buf.into_iter().collect()),
                );
            }

            println!("Dump loaded.");
            println!("{loaded_state:?}");

            // reader.seek(io::SeekFrom::Start(0)).await.unwrap();

            Ok(loaded_state)
        } else {
            Ok(HashMap::new())
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
