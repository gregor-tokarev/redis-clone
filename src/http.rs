use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Http<'a> {
    url: &'a str,
    body: String,
}

impl<'a> Http<'a> {
    pub fn new(url: &'a str, body: String) -> Self {
        Self { url, body }
    }

    pub async fn build_stream(&self) -> TcpStream {
        TcpStream::connect(self.url).await.unwrap()
    }

    pub async fn make_request(&self) -> String {
        let mut stream = self.build_stream().await;

        stream.write_all(self.body.as_bytes()).await.unwrap();

        stream.flush().await.unwrap();

        let mut buffer = vec![0u8; 512];

        let bytes_read = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8(buffer[..bytes_read].to_vec()).unwrap();
        // response.push_str(std::str::from_utf8(&buffer[..bytes_read]).unwrap());

        // println!("{response}");

        response
    }
}
