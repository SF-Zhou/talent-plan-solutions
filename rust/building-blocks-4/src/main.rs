use std::sync::{mpsc, Arc, Mutex};

trait Run {
    fn run(self: Box<Self>);
}

impl<F: FnOnce()> Run for F {
    fn run(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn Run + Send + 'static>;
type Thread = std::thread::JoinHandle<()>;

pub struct ThreadPool {
    _workers: Vec<Thread>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(threads: u32) -> ThreadPool {
        let (sender, receiver): (mpsc::Sender<Job>, mpsc::Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Thread> = Vec::with_capacity(threads as usize);
        for _ in 0..threads {
            let receiver = receiver.clone();
            workers.push(std::thread::spawn(move || loop {
                let job_ch = receiver.lock().unwrap();
                let job = job_ch.recv().unwrap();
                drop(job_ch);
                job.run()
            }));
        }
        ThreadPool {
            _workers: workers,
            sender,
        }
    }

    pub fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(job)).unwrap()
    }
}

fn main() {
    let pool = ThreadPool::new(4);
    for i in 0..4 {
        pool.spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(250 * i));
            println!("This is Task {}", i);
        });
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
}
