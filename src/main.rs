mod utils;

use std::{thread, time};

use utils::thread_pool::{ThreadPool};

fn main() {
    let thread_p = ThreadPool::new(3);
    for _ in 1..10  {
        thread_p.execute(|| println!("hhhhhhhhhhhhhhh"));
    }
    thread::sleep(time::Duration::new(2, 0));
}
