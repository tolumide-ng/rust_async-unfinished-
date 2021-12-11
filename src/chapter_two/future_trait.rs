use std::pin::Pin;
use std::task::Context;

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait SimpleFuture {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}



