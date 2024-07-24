use std::{
    sync::{Arc, Mutex},
    sync::mpsc::{channel, Sender, Receiver},
    thread,
};

type BoxedJob = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(BoxedJob),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool with a maximum number of threads.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    /// Execute a function on a worker thread.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send a terminate message to all workers.
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Wait for all workers to finish.
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
