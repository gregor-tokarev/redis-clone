use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct TcpRequest<'a> {
    url: &'a str,
    body: String,
}

impl<'a> TcpRequest<'a> {
    pub fn new(url: &'a str, body: String) -> Self {
        Self { url, body }
    }

    pub async fn build_stream(&self) -> TcpStream {
        TcpStream::connect(self.url).await.unwrap()
    }

    pub async fn make_request(&self) -> String {
        let mut stream = self.build_stream().await;

        println!("Fucking send thiiiiiiiiiis:\r\n{}", self.body);
        stream.write_all(self.body.as_bytes()).await.unwrap();

        stream.flush().await.unwrap();

        let mut buffer = [0u8; 1024];

        let bytes_read = stream.read(&mut buffer).await.unwrap();
        println!("{bytes_read}");
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        stream.shutdown().await.unwrap();

        response
    }
}
