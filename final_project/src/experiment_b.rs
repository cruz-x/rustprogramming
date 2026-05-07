use rand::prelude::*;
use rand_pcg::Pcg64;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::fs::File;
use std::io::Write;

// --- DATA STRUCTURES ---
#[derive(Debug, Clone)]
enum TaskKind { CPU, IO }

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    created_at: Instant, 
    kind: TaskKind,
    duration: Duration,
}

struct Metrics {
    total_completed: AtomicU32,
    total_wait_time_ms: AtomicU64,
    total_turnaround_time_ms: AtomicU64,
}

// --- WORKLOAD GENERATION (STRESSED) ---
fn generate_tasks(count: usize, seed: u64) -> Vec<Task> {
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut tasks = Vec::new();
    let start_time = Instant::now();

    for i in 0..count {
        let kind = if rng.gen_bool(0.5) { TaskKind::CPU } else { TaskKind::IO };
        tasks.push(Task {
            id: i as u32,
            created_at: start_time,
            kind,
            // STRESS: Tasks now take 1000ms to 3000ms (1-3 seconds)
            duration: Duration::from_millis(rng.gen_range(1000..3000)),
        });
    }
    tasks
}

fn main() {
    let num_workers = 4; 
    let total_tasks = 500;
    let seed = 42; 
    let system_start = Instant::now();

    let metrics = Arc::new(Metrics {
        total_completed: AtomicU32::new(0),
        total_wait_time_ms: AtomicU64::new(0),
        total_turnaround_time_ms: AtomicU64::new(0),
    });

    println!("!!! STARTING EXPERIMENT B: STRESSED WORKLOAD !!!");

    let tasks = generate_tasks(total_tasks, seed);
    let (tx, rx) = mpsc::channel::<Option<Task>>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];
    for worker_id in 0..num_workers {
        let rx_clone = Arc::clone(&rx);
        let m_clone = Arc::clone(&metrics);
        
        let handle = thread::spawn(move || {
            loop {
                let message = rx_clone.lock().unwrap().recv().unwrap();
                match message {
                    Some(task) => {
                        let wait_duration = Instant::now().duration_since(task.created_at);
                        println!("Worker {} [STRESS-EXEC] Task {}", worker_id, task.id);
                        thread::sleep(task.duration); 
                        
                        let turnaround = Instant::now().duration_since(task.created_at);
                        m_clone.total_completed.fetch_add(1, Ordering::SeqCst);
                        m_clone.total_wait_time_ms.fetch_add(wait_duration.as_millis() as u64, Ordering::SeqCst);
                        m_clone.total_turnaround_time_ms.fetch_add(turnaround.as_millis() as u64, Ordering::SeqCst);
                    }
                    None => break,
                }
            }
        });
        handles.push(handle);
    }

    // --- DISPATCHING (BURST ARRIVAL) ---
    for task in tasks {
        tx.send(Some(task)).unwrap();
        // STRESS: Tasks arrive every 1ms (super fast burst)
        thread::sleep(Duration::from_millis(1)); 
    }

    for _ in 0..num_workers { tx.send(None).unwrap(); }
    for handle in handles { handle.join().unwrap(); }

    let total_time = system_start.elapsed();
    let completed = metrics.total_completed.load(Ordering::SeqCst);
    let avg_wait = metrics.total_wait_time_ms.load(Ordering::SeqCst) as f64 / completed as f64;
    let avg_turnaround = metrics.total_turnaround_time_ms.load(Ordering::SeqCst) as f64 / completed as f64;

    let report = format!(
        "\n--- FINAL METRICS REPORT (EXPERIMENT B) ---\n\
        Total Tasks Completed: {}\n\
        Makespan (Total Runtime): {:?}\n\
        Average Wait Time: {:.2} ms\n\
        Average Turnaround Time: {:.2} ms\n\
        -------------------------------------------\n",
        completed, total_time, avg_wait, avg_turnaround
    );

    println!("{}", report);

    // SAVE TO B FILE
    let mut file = File::create("experiment_results_b.txt").expect("File error");
    file.write_all(report.as_bytes()).expect("Write error");
    println!("Experiment B results saved to experiment_results_B.txt");
}