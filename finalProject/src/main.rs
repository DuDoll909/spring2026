use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::thread;
use rand::Rng;

//The FIFO is the first code presented and the Optimized code is right below it (Line 463)


fn fmt_ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}



//FIFO organization


#[derive(Debug, Clone, Copy)]
enum TaskType{
    CPU,
    IO,

}

#[derive(Debug)]
struct Task{
    id: usize,
    arrival_time: Instant,
    kind: TaskType,
    duration: Duration,
}

enum Message{
    NewTask(Task),
    Terminate,

}

struct TaskStats{
    worker_id: usize,
    task_id: usize,
    kind: TaskType,
    wait_time: Duration,
    service_time: Duration,
    turnaround_time: Duration,
    finish_time: Instant,
}

struct StatsSummary {
    total_tasks: usize,
    total_wait: Duration,
    total_turnaround: Duration,
    max_wait: Duration,
    cpu_tasks: usize,
    io_tasks: usize,
    max_turnaround: Duration,
    last_finish: Option<Instant>,
    total_cpu_weighted_time: Duration,
}

impl StatsSummary {
    fn new() -> Self {
        StatsSummary {
            total_tasks: 0,
            total_wait: Duration::ZERO,
            total_turnaround: Duration::ZERO,
            max_wait: Duration::ZERO,
            cpu_tasks: 0,
            io_tasks: 0,
            max_turnaround: Duration::ZERO,
            last_finish: None,
            total_cpu_weighted_time: Duration:: ZERO,
        }
    }

    fn record(&mut self, s: TaskStats) {
    self.total_tasks += 1;
    self.total_wait += s.wait_time;
    self.total_turnaround += s.turnaround_time;
    self.max_wait = self.max_wait.max(s.wait_time);
    self.max_turnaround = self.max_turnaround.max(s.turnaround_time);

    let finish_time = s.finish_time;
    let kind = s.kind;
    self.last_finish = match self.last_finish{
        Some(t) => Some(t.max(finish_time)),
        None => Some(finish_time),
    };

    match kind {
        TaskType::CPU => self.cpu_tasks += 1,
        TaskType::IO => self.io_tasks += 1,
    };

    let cpu_fraction = match kind{
        TaskType::CPU => 0.35,
        TaskType::IO => 0.10,
    };

    self.total_cpu_weighted_time += Duration::from_secs_f64(s.service_time.as_secs_f64() * cpu_fraction);
}
}

struct Sample {
    t_ms: f64,
    cpu_used: f64,
    active_workers: usize,
}



struct Scheduler{
    sender: mpsc::Sender<Task>,
}

impl Scheduler{
    fn execute(&self, task: Task){
        self.sender.send(task).unwrap();
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{

    fn new(
        id: usize,
        receiver : mpsc::Receiver<Message>,
        idle_tx: mpsc::Sender<usize>,
        stats_tx: mpsc::Sender<TaskStats>,
        max_cpu: Arc<(Mutex<f64>, Condvar)>,
        busy_flags: Arc<Mutex<Vec<bool>>>,

    ) -> Worker{

        let thread = thread::spawn(move || {
        let _ = idle_tx.send(id);
        loop{

            let message = match receiver.recv(){
                Ok(m) => m,
                Err(_) => break,

            };

            match message{

                Message::NewTask(task) =>{
                    let start_time = Instant::now();

                    let wait_time = start_time
                        .checked_duration_since(task.arrival_time)
                        .unwrap_or(Duration::ZERO);

                    println!("Worker {} executing task {} ({:?}) with a wait time = {:?}", id, task.id, task.kind, wait_time);

                    {
                        let mut flags = busy_flags.lock().unwrap();
                        flags[id] = true;
                    }

                    thread::sleep(task.duration);

                    let cost = match task.kind{
                        TaskType::CPU => 0.35,
                        TaskType::IO => 0.10,
                    };

                    let (lock, cv) = &*max_cpu;
                    let mut used = lock.lock().unwrap();
                    *used -= cost;
                    if *used < 0.0 {
                         *used = 0.0;
                    }
                    cv.notify_all();

                    let finish_time = Instant::now();
                    let turnaround_time = finish_time
                        .checked_duration_since(task.arrival_time)
                        .unwrap_or(Duration::ZERO);

                    let service_time = finish_time - start_time;


                    println!("Worker {} finished task {} with time = {:?} and turnaround = {:?}", id, task.id, finish_time - start_time, turnaround_time);

                    let stats = TaskStats {
                    worker_id: id,
                    task_id: task.id,
                    kind: task.kind,
                    wait_time,
                    service_time,
                    turnaround_time,
                    finish_time,
                };

                stats_tx.send(stats).unwrap();
                {
                    let mut flags = busy_flags.lock().unwrap();
                    flags[id] = false;
                }
                let _ = idle_tx.send(id);

                

                }

                

                Message::Terminate =>{
                    println!("Worker {} got terminated, ty for your work!", id);
                    break;
                }

            }
        }

   });

   Worker{
    id,
    thread: Some(thread),

   }

    }

}

fn main() {

let sim_start = Instant::now();
let (stats_tx, stats_rx) = mpsc::channel::<TaskStats>();
let (idle_tx, idle_rx) = mpsc::channel::<usize>();
let (ready_tx, ready_rx) = mpsc::channel::<Task>();
let max_cpu = Arc::new((Mutex::new(0.0_f64), Condvar::new()));


let mut workers = Vec::new();
let num_workers = 8;
let busy_flags = Arc::new(Mutex::new(vec![false; num_workers]));

let mut worker_txs: Vec<mpsc::Sender<Message>> = Vec::new();
for id in 0..num_workers{
    let (tx, rx) = mpsc::channel::<Message>();
    worker_txs.push(tx);
    workers.push(Worker::new(
        id,
        rx,
        idle_tx.clone(),
        stats_tx.clone(),
        Arc::clone(&max_cpu),
        Arc::clone(&busy_flags),
    ));
}

drop(stats_tx);

let dispatcher_cpu = Arc::clone(&max_cpu);

let dispatcher_handle = thread::spawn(move || {
    let mut idle_workers: VecDeque<usize> = VecDeque::new();
    while let Ok(task) = ready_rx.recv(){

           let cost = match task.kind{
                TaskType::CPU => 0.35,
                TaskType::IO => 0.10,
            };

            let (lock,cv) =&*dispatcher_cpu;
            let mut used = lock.lock().unwrap();
            const EPS: f64 = 1e-9;
            while *used + cost > 1.0 + EPS{
                used = cv.wait(used).unwrap();
            }
            *used += cost;
            drop(used);

            let worker_id = loop{
                if let Some(id) = idle_workers.pop_front() {
                    break id;
                }
                let id = idle_rx.recv().unwrap();
                idle_workers.push_back(id);
            };
           if worker_txs[worker_id].send(Message::NewTask(task)).is_err() {
                let (lock, cv) = &*dispatcher_cpu;
                let mut used = lock.lock().unwrap_or_else(|e| e.into_inner());
                *used -= cost;
                if *used < 0.0 { *used = 0.0; }
                cv.notify_all();
                continue;
            }
    }

   for tx in worker_txs{
    let _ = tx.send(Message::Terminate);
   }

});

let (stop_tx, stop_rx) = mpsc::channel::<()>();

let monitor_cpu = Arc::clone(&max_cpu);
let monitor_busy = Arc::clone(&busy_flags);
let monitor_start = sim_start;

let monitor_handle = thread::spawn(move || {
    let mut samples: Vec<Sample> = Vec::new();

    loop {

        if stop_rx.try_recv().is_ok() {
            break;
        }

        let t = Instant::now().duration_since(monitor_start);

        let cpu_used = {
            let (lock, _) = &*monitor_cpu;
            *lock.lock().unwrap()
        };

        let active_workers = {
            let flags = monitor_busy.lock().unwrap();
            flags.iter().filter(|&&b| b).count()
        };

        samples.push(Sample {
            t_ms: fmt_ms(t),
            cpu_used,
            active_workers,
        });

        thread::sleep(Duration::from_millis(10));
    }

    samples
});




let generator_handle = thread::spawn(move || {

let scheduler = Scheduler{
    sender: ready_tx,
};

let mut rng = rand::thread_rng();
let num_tasks = 1000;
let arrival_interval = Duration::from_millis(20);

for i in 0..num_tasks{

    let roll: u8 = rng.gen_range(0..100);
    let kind = if roll < 70{
        TaskType::IO
    }
    else{
        TaskType::CPU
    };


    let task = Task{
        id: i,
        arrival_time: Instant::now(),
        kind,
        duration: Duration::from_millis(200),
    };

    scheduler.execute(task);
    thread::sleep(arrival_interval);
}

});

generator_handle.join().unwrap();
dispatcher_handle.join().unwrap();

for worker in &mut workers{
    if let Some(handle) = worker.thread.take(){
        handle.join().unwrap();
    }
}

let _ = stop_tx.send(());
let samples = monitor_handle.join().unwrap();

let denom = samples.len().max(1) as f64;

let avg_cpu = samples.iter().map(|s| s.cpu_used).sum::<f64>() / denom;

let avg_worker_activity = samples.iter()
    .map(|s| s.active_workers as f64 / num_workers as f64)
    .sum::<f64>() / denom;

// total worker-time spent working (approx; 10ms per sample)
let total_worker_time_ms = samples.iter()
    .map(|s| s.active_workers as f64 * 10.0)
    .sum::<f64>();

println!("\n--- Monitor Metrics (10ms sampling) ---");
println!("Average CPU consumption: {:.2}%", avg_cpu * 100.0);
println!("Average worker activity: {:.2}%", avg_worker_activity * 100.0);
println!("Total worker-time spent working: {:.2} ms", total_worker_time_ms);


let sim_end = Instant::now();
let total_runtime = sim_end - sim_start;
let mut summary = StatsSummary::new();
while let Ok(stat) = stats_rx.recv() {
    summary.record(stat);
}

let avg_cpu_usage = if total_runtime.as_secs_f64() > 0.0 {
    summary.total_cpu_weighted_time.as_secs_f64() / total_runtime.as_secs_f64()
} else {
    0.0
};




println!("=== Summary ===");
println!("Total tasks completed: {}", summary.total_tasks);

let n = summary.total_tasks as f64;
if summary.total_tasks > 0 {
    println!(
        "Average wait time: {:.2} ms", fmt_ms(summary.total_wait) / n
    );
    println!("Average turnaround time: {:.2} ms", fmt_ms(summary.total_turnaround) / n
    );
}

println!("Max wait time: {:.2} ms", fmt_ms(summary.max_wait));
println!("CPU tasks completed: {}", summary.cpu_tasks);
println!("IO tasks completed: {}", summary.io_tasks);
println!("Total runtime: {:.2} ms", fmt_ms(total_runtime));
println!("Average CPU usage: {:.2}%", avg_cpu_usage * 100.0);
if let Some(last) = summary.last_finish{
    println!("Makespan: {:.2} ms", fmt_ms(last - sim_start));
}
println!("Max turnaround time: {:.2} ms",fmt_ms(summary.max_turnaround))



}





/*


//Optimized Code

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskType{
    CPU,
    IO,

}

#[derive(Debug)]
struct Task{
    id: usize,
    arrival_time: Instant,
    kind: TaskType,
    duration: Duration,
}

enum Message{
    NewTask(Task),
    Terminate,

}

struct TaskStats{
    worker_id: usize,
    task_id: usize,
    kind: TaskType,
    wait_time: Duration,
    service_time: Duration,
    turnaround_time: Duration,
    finish_time: Instant,
}

struct StatsSummary {
    total_tasks: usize,
    total_wait: Duration,
    total_turnaround: Duration,
    max_wait: Duration,
    cpu_tasks: usize,
    io_tasks: usize,
    max_turnaround: Duration,
    last_finish: Option<Instant>,
    total_cpu_weighted_time: Duration,
}

impl StatsSummary {
    fn new() -> Self {
        StatsSummary {
            total_tasks: 0,
            total_wait: Duration::ZERO,
            total_turnaround: Duration::ZERO,
            max_wait: Duration::ZERO,
            cpu_tasks: 0,
            io_tasks: 0,
            max_turnaround: Duration::ZERO,
            last_finish: None,
            total_cpu_weighted_time: Duration:: ZERO,
        }
    }

    fn record(&mut self, s: TaskStats) {
    self.total_tasks += 1;
    self.total_wait += s.wait_time;
    self.total_turnaround += s.turnaround_time;
    self.max_wait = self.max_wait.max(s.wait_time);
    self.max_turnaround = self.max_turnaround.max(s.turnaround_time);

    let finish_time = s.finish_time;
    let kind = s.kind;
    self.last_finish = match self.last_finish{
        Some(t) => Some(t.max(finish_time)),
        None => Some(finish_time),
    };

    match kind {
        TaskType::CPU => self.cpu_tasks += 1,
        TaskType::IO => self.io_tasks += 1,
    };

    let cpu_fraction = match kind{
        TaskType::CPU => 0.35,
        TaskType::IO => 0.10,
    };

    self.total_cpu_weighted_time += Duration::from_secs_f64(s.service_time.as_secs_f64() * cpu_fraction);
}
}

struct Sample {
    t_ms: f64,
    cpu_used: f64,
    active_workers: usize,
}



struct Scheduler{
    sender: mpsc::Sender<Task>,
}

impl Scheduler{
    fn execute(&self, task: Task){
        self.sender.send(task).unwrap();
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{

    fn new(
        id: usize,
        receiver : mpsc::Receiver<Message>,
        idle_tx: mpsc::Sender<usize>,
        stats_tx: mpsc::Sender<TaskStats>,
        max_cpu: Arc<(Mutex<f64>, Condvar)>,
        busy_flags: Arc<Mutex<Vec<bool>>>,

    ) -> Worker{

        let thread = thread::spawn(move || {
        let _ = idle_tx.send(id);
        loop{

            let message = match receiver.recv(){
                Ok(m) => m,
                Err(_) => break,

            };

            match message{

                Message::NewTask(task) =>{
                    let start_time = Instant::now();

                    let wait_time = start_time
                        .checked_duration_since(task.arrival_time)
                        .unwrap_or(Duration::ZERO);

                    println!("Worker {} executing task {} ({:?}) with a wait time = {:?}", id, task.id, task.kind, wait_time);

                    {
                        let mut flags = busy_flags.lock().unwrap();
                        flags[id] = true;
                    }

                    thread::sleep(task.duration);

                    let cost = match task.kind{
                        TaskType::CPU => 0.35,
                        TaskType::IO => 0.10,
                    };

                    let (lock, cv) = &*max_cpu;
                    let mut used = lock.lock().unwrap();
                    *used -= cost;
                    if *used < 0.0 {
                         *used = 0.0;
                    }
                    cv.notify_all();

                    let finish_time = Instant::now();
                    let turnaround_time = finish_time
                        .checked_duration_since(task.arrival_time)
                        .unwrap_or(Duration::ZERO);

                    let service_time = finish_time - start_time;


                    println!("Worker {} finished task {} with time = {:?} and turnaround = {:?}", id, task.id, finish_time - start_time, turnaround_time);

                    let stats = TaskStats {
                    worker_id: id,
                    task_id: task.id,
                    kind: task.kind,
                    wait_time,
                    service_time,
                    turnaround_time,
                    finish_time,
                };

                stats_tx.send(stats).unwrap();
                {
                    let mut flags = busy_flags.lock().unwrap();
                    flags[id] = false;
                }
                let _ = idle_tx.send(id);

                

                }

                

                Message::Terminate =>{
                    println!("Worker {} got terminated, ty for your work!", id);
                    break;
                }

            }
        }

   });

   Worker{
    id,
    thread: Some(thread),

   }

    }

}

fn main() {

let sim_start = Instant::now();
let (stats_tx, stats_rx) = mpsc::channel::<TaskStats>();
let (idle_tx, idle_rx) = mpsc::channel::<usize>();
let (ready_tx, ready_rx) = mpsc::channel::<Task>();
let max_cpu = Arc::new((Mutex::new(0.0_f64), Condvar::new()));


let mut workers = Vec::new();
let num_workers = 8;
let busy_flags = Arc::new(Mutex::new(vec![false; num_workers]));

let mut worker_txs: Vec<mpsc::Sender<Message>> = Vec::new();
for id in 0..num_workers{
    let (tx, rx) = mpsc::channel::<Message>();
    worker_txs.push(tx);
    workers.push(Worker::new(
        id,
        rx,
        idle_tx.clone(),
        stats_tx.clone(),
        Arc::clone(&max_cpu),
        Arc::clone(&busy_flags),
    ));
}

drop(stats_tx);

let dispatcher_cpu = Arc::clone(&max_cpu);

let dispatcher_handle = thread::spawn(move || {
    const EPS: f64 = 1e-9;

    let mut pending_cpu: VecDeque<Task> = VecDeque::new();
    let mut pending_io: VecDeque<Task>  = VecDeque::new();
    let mut idle_workers: VecDeque<usize> = VecDeque::new();

    while let Ok(task) = ready_rx.recv() {
        // enqueue by type
        match task.kind {
            TaskType::CPU => pending_cpu.push_back(task),
            TaskType::IO  => pending_io.push_back(task),
        }

        // try to dispatch immediately
        loop {
            // get idle workers
            while let Ok(id) = idle_rx.try_recv() {
                idle_workers.push_back(id);
            }
            if idle_workers.is_empty() {
                break;
            }

            // check remaining CPU
            let used = {
                let (lock, _) = &*dispatcher_cpu;
                *lock.lock().unwrap_or_else(|e| e.into_inner())
            };
            let remaining = 1.0 - used;

            // pick next task that fits
            let task = if remaining + EPS >= 0.35 {
                pending_cpu.pop_front()
            } else if remaining + EPS >= 0.10 {
                pending_io.pop_front()
            } else {
                None
            };

            let task = match task {
                Some(t) => t,
                None => break,
            };

            let cost = if task.kind == TaskType::CPU { 0.35 } else { 0.10 };

            // reserve CPU
            {
                let (lock, cv) = &*dispatcher_cpu;
                let mut used = lock.lock().unwrap_or_else(|e| e.into_inner());
                while *used + cost > 1.0 + EPS {
                    used = cv.wait(used).unwrap_or_else(|e| e.into_inner());
                }
                *used += cost;
            }

            // assign worker
            let wid = idle_workers.pop_front().unwrap();
            if worker_txs[wid].send(Message::NewTask(task)).is_err() {
                // rollback CPU if worker vanished
                let (lock, cv) = &*dispatcher_cpu;
                let mut used = lock.lock().unwrap_or_else(|e| e.into_inner());
                *used = (*used - cost).max(0.0);
                cv.notify_all();
            }
        }
    }

    // After input closes, keep dispatching remaining work
    while !pending_cpu.is_empty() || !pending_io.is_empty() {

    // pull newly idle workers
    while let Ok(id) = idle_rx.try_recv() {
        idle_workers.push_back(id);
    }

    // try to dispatch (same logic as main loop)
    let used = {
        let (lock, _) = &*dispatcher_cpu;
        *lock.lock().unwrap_or_else(|e| e.into_inner())
    };
    let remaining = 1.0 - used;

    let next_task = if remaining >= 0.35 && !pending_cpu.is_empty() {
        pending_cpu.pop_front()
    } else if remaining >= 0.10 && !pending_io.is_empty() {
        pending_io.pop_front()
    } else {
        None
    };

    if let Some(task) = next_task {
        let cost = if task.kind == TaskType::CPU { 0.35 } else { 0.10 };

        {
            let (lock, cv) = &*dispatcher_cpu;
            let mut used = lock.lock().unwrap_or_else(|e| e.into_inner());
            while *used + cost > 1.0  + EPS{
                used = cv.wait(used).unwrap_or_else(|e| e.into_inner());
            }
            *used += cost;
        }

        let worker_id = match idle_workers.pop_front() {
            Some(id) => id,
            None => continue,
        };

        if worker_txs[worker_id].send(Message::NewTask(task)).is_err() {
            let (lock, cv) = &*dispatcher_cpu;
            let mut used = lock.lock().unwrap_or_else(|e| e.into_inner());
            *used = (*used - cost).max(0.0);
            cv.notify_all();
        }
    } else {
        // nothing fits: CPU is the limiter, so wait for CPU to change
        let (lock, cv) = &*dispatcher_cpu;
        let used = lock.lock().unwrap_or_else(|e| e.into_inner());
        let _guard = cv.wait(used).unwrap_or_else(|e| e.into_inner());

    }
}

    for tx in worker_txs {
        let _ = tx.send(Message::Terminate);
    }
});

let (stop_tx, stop_rx) = mpsc::channel::<()>();

let monitor_cpu = Arc::clone(&max_cpu);
let monitor_busy = Arc::clone(&busy_flags);
let monitor_start = sim_start;

let monitor_handle = thread::spawn(move || {
    let mut samples: Vec<Sample> = Vec::new();

    loop {

        if stop_rx.try_recv().is_ok() {
            break;
        }

        let t = Instant::now().duration_since(monitor_start);

        let cpu_used = {
            let (lock, _) = &*monitor_cpu;
            *lock.lock().unwrap()
        };

        let active_workers = {
            let flags = monitor_busy.lock().unwrap();
            flags.iter().filter(|&&b| b).count()
        };

        samples.push(Sample {
            t_ms: fmt_ms(t),
            cpu_used,
            active_workers,
        });

        thread::sleep(Duration::from_millis(10));
    }

    samples
});




let generator_handle = thread::spawn(move || {

let scheduler = Scheduler{
    sender: ready_tx,
};

let mut rng = rand::thread_rng();
let num_tasks = 1000;
let arrival_interval = Duration::from_millis(20);

for i in 0..num_tasks{

    let roll: u8 = rng.gen_range(0..100);
    let kind = if roll < 70{
        TaskType::IO
    }
    else{
        TaskType::CPU
    };


    let task = Task{
        id: i,
        arrival_time: Instant::now(),
        kind,
        duration: Duration::from_millis(200),
    };

    scheduler.execute(task);
    thread::sleep(arrival_interval);
}

});

generator_handle.join().unwrap();
dispatcher_handle.join().unwrap();

for worker in &mut workers{
    if let Some(handle) = worker.thread.take(){
        handle.join().unwrap();
    }
}

let _ = stop_tx.send(());
let samples = monitor_handle.join().unwrap();

let denom = samples.len().max(1) as f64;

let avg_cpu = samples.iter().map(|s| s.cpu_used).sum::<f64>() / denom;

let avg_worker_activity = samples.iter()
    .map(|s| s.active_workers as f64 / num_workers as f64)
    .sum::<f64>() / denom;

// total worker-time spent working (approx; 10ms per sample)
let total_worker_time_ms = samples.iter()
    .map(|s| s.active_workers as f64 * 10.0)
    .sum::<f64>();

println!("\n--- Monitor Metrics (10ms sampling) ---");
println!("Average CPU consumption: {:.2}%", avg_cpu * 100.0);
println!("Average worker activity: {:.2}%", avg_worker_activity * 100.0);
println!("Total worker-time spent working: {:.2} ms", total_worker_time_ms);


let sim_end = Instant::now();
let total_runtime = sim_end - sim_start;
let mut summary = StatsSummary::new();
while let Ok(stat) = stats_rx.recv() {
    summary.record(stat);
}

let avg_cpu_usage = if total_runtime.as_secs_f64() > 0.0 {
    summary.total_cpu_weighted_time.as_secs_f64() / total_runtime.as_secs_f64()
} else {
    0.0
};




println!("=== Summary ===");
println!("Total tasks completed: {}", summary.total_tasks);

let n = summary.total_tasks as f64;
if summary.total_tasks > 0 {
    println!(
        "Average wait time: {:.2} ms", fmt_ms(summary.total_wait) / n
    );
    println!("Average turnaround time: {:.2} ms", fmt_ms(summary.total_turnaround) / n
    );
}

println!("Max wait time: {:.2} ms", fmt_ms(summary.max_wait));
println!("CPU tasks completed: {}", summary.cpu_tasks);
println!("IO tasks completed: {}", summary.io_tasks);
println!("Total runtime: {:.2} ms", fmt_ms(total_runtime));
println!("Average CPU usage: {:.2}%", avg_cpu_usage * 100.0);
if let Some(last) = summary.last_finish{
    println!("Makespan: {:.2} ms", fmt_ms(last - sim_start));
}
println!("Max turnaround time: {:.2} ms",fmt_ms(summary.max_turnaround))



}
*/