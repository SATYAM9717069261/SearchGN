use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::SendError;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    sender: Sender<Job>,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        let (sender, receiver): (Sender<Job>, Receiver<Job>) = std::sync::mpsc::channel();

        let thread = thread::spawn(move || {
            println!("Worker {} started", id);
            while let Ok(job) = receiver.recv() {
                println!("Worker {} received job", id);
                job();
            }
            println!("Worker {} stopped", id);
        });

        Self { id, sender, thread: Some(thread), }
    }

    pub fn submit(&self, job: Job) ->  Result<(), SendError<Job>> {
        self.sender.send(job)
    }
}
