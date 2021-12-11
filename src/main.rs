pub mod chapter_one;
pub mod chapter_two;

use futures::executor::block_on;

use chapter_one::one;

fn main() {
    block_on(one::async_main())
}
