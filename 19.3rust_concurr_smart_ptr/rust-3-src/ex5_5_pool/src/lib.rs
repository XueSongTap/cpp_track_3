use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Task(Job),
    Terminate,
}

// all workers get task from it
type SharedTaskReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

struct Worker {
    id: usize,
    // task return type is empty '()'
    // when drop, some threads which joined first will not own JoinHandle
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, task_receiver: SharedTaskReceiver) -> Worker {
        let handle = std::thread::spawn(move || loop {
            let message = task_receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Task(job) => {
                    println!("Worker {} got a job, executing...", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id: id,
            handle: Some(handle),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    task_sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("A thread pool with 0 thread is not allowed");
        }

        let (sender, receiver) = mpsc::channel();
        let shared_receiver = Arc::new(Mutex::new(receiver));

        let mut workers = vec![];
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&shared_receiver)));
        }

        Ok(ThreadPool {
            workers: workers,
            task_sender: sender,
        })
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, func: F) {
        let new_job = Message::Task(Box::new(func));
        self.task_sender.send(new_job).unwrap();
    }
}
// 为 ThreadPool 实现 Drop trait 来 join 所有的线程，保证队列中的任务完成。
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!(
            "Sending terminate message to {} workers",
            self.workers.len()
        );
        for _ in self.workers.iter() {
            self.task_sender.send(Message::Terminate).unwrap();
        }

        for worker in self.workers.iter_mut() {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}