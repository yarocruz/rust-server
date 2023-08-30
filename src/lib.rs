use std::{sync::{mpsc, Arc, Mutex},
 thread,
};
#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub name: String,
    pub done: bool,
}

impl Todo {
   pub fn new(id: u32, name: String, done: bool) -> Todo {
    Todo { id, name, done, }
   } 
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new Thread Pool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The new function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // shadow receiv and wrap into a "smart pointer"
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
    
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
   fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
        let message = receiver.lock().unwrap().recv();

        match message {
           Ok(job) => {
            println!("Worker {id} got a job; execucting.");
            job();
           } 
           Err(_) => {
            println!("Worker {id} disconnected; shuttingdown.");
            break;
           }
        }
    });

    Worker { id, thread: Some(thread) }
   } 
}