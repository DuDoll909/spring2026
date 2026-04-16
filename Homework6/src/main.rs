/*


//Assignment 3 of Homework 6
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        let (sender, reciever) = mpsc::channel();
        
        // TODO: Create and store workers
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(size);

        for  id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&reciever)));
        }
        
        // TODO: Return the ThreadPool
        ThreadPool{ workers, sender }
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        
        // TODO: Wait for all workers to finish
        for worker in &mut self.workers{
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || {
            loop {
                let message = receiver
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
            }
    }

        });
        
        // TODO: Return the Worker
    Worker {
        id,
        thread: Some(thread),
    }
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}


*/

//Assignment 4 of Homework 6


use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx,rx) = mpsc::channel::<i32>();
    let rx = Arc::new(Mutex::new(rx));

    // TODO: Create 2 producer threads
    let mut producer_handles = Vec::new();
    
    for id in 0..2{
        let producer_tx = tx.clone();

        let handle = thread::spawn(move || {
            producer(id, producer_tx, ITEM_COUNT / 2)
        });
        
        producer_handles.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    let mut consumer_handles = Vec::new();

    for id in 0..3{
        let consumer_rx = rx.clone();

        let handle = thread::spawn(move || {
            consumer(id,consumer_rx,)
        });

        consumer_handles.push(handle);
    }


    
    // TODO: Wait for all threads to finish
    for handle in producer_handles{
        handle.join().unwrap();
    }

    for _ in 0..3{
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    for handle in consumer_handles{
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();

    for _ in 0..item_count{
        let value = rng.gen_range(1..=100);

        println!("Producer {} produced value {}", id, value);
        tx.send(value).unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {} finished producing", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop{
        let value = {
            let rx_lock = rx.lock().unwrap();
            rx_lock.recv().unwrap()
        };

        if value == TERMINATION_SIGNAL{
            println!("Consumer {} recieved termination signal, exiting!", id);
            break;
        }

        println!("Consumer {} consumed value {}!", id, value);

        thread::sleep(Duration::from_millis(100));
    }
}
