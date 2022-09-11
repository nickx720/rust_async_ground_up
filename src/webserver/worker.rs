use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Task {
    New(Job),
    Exit,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let task = {
                let rx = receiver.lock().unwrap();
                rx.recv().unwrap()
            };

            match task {
                Task::New(job) => job(),
                Task::Exit => break,
            }
        });

        Worker {
            id,
            thread: Some(handle),
        }
    }
}
