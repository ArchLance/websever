
use std::sync::Mutex;
use std::thread::{JoinHandle, spawn};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver:Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = Some(spawn(move || {
            //如果没有停止信号，线程会无限期loop，由于实现了drop导致主线程会等待所有线程结束而一直堵塞
            loop {   
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing...", id);
                        job();       
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate...", id);
                        break;
                    }
                } 
            }
        }));
        Worker{
            id,
            thread
        }
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending terminnate message to all workers...");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// 创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// # Panics
    /// 
    /// `new`函数会在size为0时触发panic
    pub fn new(size: usize) -> Self{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool{
            workers,
            sender
        }
    }
    /// 执行任务
    /// 
    /// 
    /// 
    /// `execute`函数会将任务发送给线程执行
    /// 
    /// 
    /// 
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}