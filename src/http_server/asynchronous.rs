use std::fs;
use std::marker::Unpin;
use std::time::Duration;
use async_std::io::{Read, Write};
use async_std::prelude::*;
use async_std::task;
use async_std::net::TcpStream;



pub async fn handle_connection<T>(mut stream: T) 
where T: Read + Write + Unpin {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1. 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write responde back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{} {}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}


#[test]
fn test() {
    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};

    use std::{
        cmp::min,
        pin::Pin
    };

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>
    }

    impl Read for MockTcpStream {
        fn poll_read(self: Pin<&mut Self>, _: &mut Context, buf: &mut [u8]) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buff.len());
            buff[..size].copy_from_slice(&self.read_data(&self.))
        }
    }
}