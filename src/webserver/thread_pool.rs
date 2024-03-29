use crate::webserver::worker::{Task, Worker};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Threadpool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Task::Exit).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(handle) = worker.thread.take() {
                handle.join().unwrap();
            }
        }
    }
}

impl Threadpool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Threadpool {
            workers,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Task::New(job)).unwrap();
    }
}
