use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //

    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Port already in use");

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut s, _)) => {
                tokio::spawn(async move {
                    loop {
                        let mut buf = [0u8; 512];

                        s.read(&mut buf).await.unwrap();
                        s.write_all(b"+PONG\r\n").await.unwrap();
                    }
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             let mut buff = [0u8; 512];
    //             stream.read(&mut buff).unwrap();
    //
    //             for _line in str::from_utf8(&buff).unwrap().lines() {
    //                 stream.write_all(b"+PONG\r\n").unwrap();
    //                 stream.flush().unwrap();
    //             }
    //             // while let Some(_line) = str::from_utf8(&buff).unwrap().lines() {
    //             //     stream.write_all(b"+PONG\r\n").unwrap();
    //             //     stream.flush().unwrap()
    //             // }
    //                          }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}
