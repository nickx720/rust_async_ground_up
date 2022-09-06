use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
