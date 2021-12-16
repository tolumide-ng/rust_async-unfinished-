use async_std::net::TcpListener;
use async_std::prelude::*;
// pub mod chapter_one;
// pub mod chapter_two;
// pub mod others;
// pub mod pinning;
pub mod http_server;

use http_server::asynchronous::handle_connection;
// use std::time::Duration;

// use chapter_two::executor::new_executor_and_spawner;

// use crate::chapter_two::timer_future::TimerFuture;



#[async_std::main]
async fn main() {
    // let (executor, spawner) = new_executor_and_spawner();

    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream.unwrap();
        handle_connection(stream).await;
    }

    // listener
    //     .incoming()
    //     .for_each_concurrent(None, |tcpstream| async move {
    //     let tcpstream = tcpstream.unwrap();
    //     handle_connection(tcpstream).await;
    // })
    // .await;


    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream)
    // }

    // spawner.spawn(async {
    //     println!("howdy!");
    //     TimerFuture::new(Duration::new(2, 0)).await;
    //     println!("done!");
    // });

    // drop(spawner);
    // executor.run();
}
