use std::sync::{Arc, Mutex};

use web_framework_rust::utils::thread_pool;

#[test]
fn thread_size_test() {
    let thread = thread_pool::ThreadPool::new(10);
    assert_eq!(thread.workers.len(), 10)
}
