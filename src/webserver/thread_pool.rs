pub struct Threadpool {
    workers: Vec<Worker>,
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(handle) = worker.thread.take() {
                handle.join().unwrap();
            }
        }
    }
}
