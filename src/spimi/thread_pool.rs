use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Shutdown,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        assert!(thread_count > 0);

        let (sender, receiver) = mpsc::channel::<Message>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(thread_count);

        for id in 0..thread_count {
            let receiver = Arc::clone(&receiver);

            let thread = thread::spawn(move || {
                loop {
                    let message = {
                        let receiver = receiver.lock().unwrap();

                        receiver.recv()
                    };

                    match message {
                        Ok(Message::NewJob(job)) => {
                            println!("Worker {id} started job");

                            job();

                            println!("Worker {id} finished job");
                        }

                        Ok(Message::Shutdown) => {
                            println!("Worker {id} shutting down");
                            break;
                        }

                        Err(_) => {
                            println!("Worker {id} disconnected");
                            break;
                        }
                    }
                }
            });

            workers.push(Worker {
                id,
                thread: Some(thread),
            });
        }

        Self {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Message::NewJob(Box::new(job)))
            .expect("Failed to send job to thread pool");
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender
                .send(Message::Shutdown)
                .expect("Failed to shutdown worker");
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .expect("Failed to join worker thread");
            }
        }
    }
}
