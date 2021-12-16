use std::net::{TcpListener, TcpStream};
// pub mod chapter_one;
// pub mod chapter_two;
// pub mod others;
// pub mod pinning;
pub mod http_server;

use http_server::synchronous::handle_connection;
// use std::time::Duration;

// use chapter_two::executor::new_executor_and_spawner;

// use crate::chapter_two::timer_future::TimerFuture;




fn main() {
    // let (executor, spawner) = new_executor_and_spawner();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }

    // spawner.spawn(async {
    //     println!("howdy!");
    //     TimerFuture::new(Duration::new(2, 0)).await;
    //     println!("done!");
    // });

    // drop(spawner);
    // executor.run();
}
