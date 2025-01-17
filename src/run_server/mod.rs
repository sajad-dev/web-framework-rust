use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread, time,
};

use crate::utils::thread_pool::ThreadPool;

pub fn handel_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");
    thread::sleep(time::Duration::from_secs(10));
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nHi");
    stream.write_all(response.as_bytes()).unwrap();
}

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
