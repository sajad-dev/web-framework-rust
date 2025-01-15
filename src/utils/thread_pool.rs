use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: Sender<Job>,
}

struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, reviver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(reviver));
        let mut workers = Vec::with_capacity(size);

        for id in 1..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers: workers,
            sender: sender,
        }
    }
    pub fn execute<T>(&self, job: T)
    where
        T: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(job)).unwrap()
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job()
        });
        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
