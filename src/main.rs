pub mod chapter_one;
pub mod chapter_two;



use std::time::Duration;

use chapter_two::executor::new_executor_and_spawner;

use crate::chapter_two::timer_future::TimerFuture;




fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);
    executor.run();
}
