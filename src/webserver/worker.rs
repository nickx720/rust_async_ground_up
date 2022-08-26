use std::thread;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize) -> Self {
        let handle = thread::spawn(|| {});

        Worker {
            id,
            thread: Some(handle),
        }
    }
}
