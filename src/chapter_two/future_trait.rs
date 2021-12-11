pub enum Poll<T> {
    Ready(T),
    Pending,
}
pub trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}


