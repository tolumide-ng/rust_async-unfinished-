use std::{pin::Pin, task::Context};

use crate::chapter_two::future_trait::{Poll, SimpleFuture};

struct Socket {}

impl Socket {
    fn has_data_to_read(&self) -> bool {
        true
    }

    fn data_to_read(&self) -> Vec<u8> {
        vec![10, 14]
    }

    fn set_readable_callback(&self, wake: fn()) {}
}



pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(self: Pin<&mut Self>, wake: &mut Context<'_>) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.data_to_read())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}