use crate::spimi::worker::{Job, Worker};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Arc<Worker>>,
    active_worker: Arc<Mutex<Vec<bool>>>,
}

impl ThreadPool {
    pub fn new(count: usize) -> Self {
        let mut workers = Vec::with_capacity(count);

        for id in 0..count {
            workers.push(Arc::new(Worker::new(id)));
        }

        Self {
            workers:workers,
            active_worker: Arc::new(Mutex::new(vec![false; count])),
        }
    }

    pub fn get_active_worker_count(&self) -> usize {
        let active_worker = self.active_worker.lock().unwrap();
        active_worker
            .iter()
            .filter(|active| **active)
            .count()
    }

    pub fn submit<F, O>( &mut self, file1: PathBuf, file2: PathBuf, process_fn: F, output_write_fn: O,) -> bool
    where
        F: FnOnce(PathBuf, PathBuf, u32) -> Option<PathBuf> + Send + 'static,
        O: FnOnce(PathBuf) + Send + 'static, {

        let worker_idx = {
            let mut active_worker = self.active_worker.lock().unwrap();
            match active_worker.iter().position(|active| !*active) {
                Some(idx) => {
                    active_worker[idx] = true;
                    idx
                },
                None => {
                    return false;
                }
            }
        };

        let active_worker = Arc::clone(&self.active_worker);
        let job: Job = Box::new(move || {
            if let Some(output) = process_fn(file1, file2,worker_idx as u32){
                output_write_fn(output);
            }
            // Worker becomes free on both cases
            let mut active_worker = active_worker.lock().unwrap();
            active_worker[worker_idx] = false;
        });

        let worker = Arc::clone(&self.workers[worker_idx]);
        match worker.submit(job){
            Ok(()) => true,
            Err(err) => {
                eprintln!("Worker {} is dead: {:?}", worker_idx, err);
                self.workers[worker_idx] = Arc::new(Worker::new(worker_idx));
                false
            }
        }
    }
}
