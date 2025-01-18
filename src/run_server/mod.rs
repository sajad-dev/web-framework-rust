pub mod handel_connection;

use std::{
    env,
    net::{TcpListener, TcpStream},
    thread, time,
};

use handel_connection::handel_connection;

use crate::core::utils::thread_pool::ThreadPool;



pub fn run_server() {
    let thread_p: ThreadPool = ThreadPool::new(80);
    let listen = TcpListener::bind(format!(
        "{}:{}",
        env::var("ADDRESS").unwrap(),
        env::var("PORT").unwrap()
    ))
    .unwrap();

    log::info!("Run server : 127.0.0.1");

    for stream in listen.incoming() {
        thread_p.execute(|| {
            let stream = stream.unwrap();
            handel_connection(stream)
        });
    }
}
