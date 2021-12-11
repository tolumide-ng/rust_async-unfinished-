use crate::chapter_two::future_trait::{SimpleFuture, Poll};

pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: Option<FutureB>,
}


// impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
// where
//     FutureA: SimpleFuture<Output = ()>,
//     FutureB: SimpleFuture<Output = ()>,
// {
//     type Output = ();
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if let Some(first) = &mut self.first {
//             match first.poll(wake) {
//                 // We've completed the first future -- remove it and start on
//                 // the second!
//                 Poll::Ready(()) => self.first.take(),
//                 // We couldn't yet complete the first future.
//                 Poll::Pending => return Poll::Pending,
//             };
//         }
//         // Now that the first future is done, attempt to complete the second.
//         self.second.unwrap().poll(wake)
//     }
// }
