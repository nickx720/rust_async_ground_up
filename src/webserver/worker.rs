use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<_>>>) -> Self {
        let handle = thread::spawn(move || loop {});

        Worker {
            id,
            thread: Some(handle),
        }
    }
}
