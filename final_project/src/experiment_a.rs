use rand::prelude::*;
use rand_pcg::Pcg64;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::fs::File;
use std::io::Write;

// --- 1. DATA STRUCTURES (Requirement #1) ---
#[derive(Debug, Clone)]
enum TaskKind { CPU, IO }

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    created_at: Instant, 
    kind: TaskKind,
    duration: Duration,
}

// Metrics container using Atomics for thread-safe calculation (Requirement #7)
struct Metrics {
    total_completed: AtomicU32,
    total_wait_time_ms: AtomicU64,
    total_turnaround_time_ms: AtomicU64,
}

// --- 2. WORKLOAD GENERATION (Requirement #2) ---
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
            // Experiment A: Balanced (100ms-500ms)
            // Experiment B: Stress (Change to 500ms-2000ms)
            duration: Duration::from_millis(rng.gen_range(100..500)),
        });
    }
    tasks
}

fn main() {
    // --- CONFIGURATION ---
    let num_workers = 4; // Requirement #3: Bounded worker pool
    let total_tasks = 500;
    let seed = 42; // Requirement #2: Fixed seed
    let system_start = Instant::now();

    let metrics = Arc::new(Metrics {
        total_completed: AtomicU32::new(0),
        total_wait_time_ms: AtomicU64::new(0),
        total_turnaround_time_ms: AtomicU64::new(0),
    });

    println!("Dispatcher starting with {} workers...", num_workers);

    let tasks = generate_tasks(total_tasks, seed);

    // --- 3. QUEUE ARCHITECTURE (Requirement #4) ---
    let (tx, rx) = mpsc::channel::<Option<Task>>();
    let rx = Arc::new(Mutex::new(rx));

    // --- 4. WORKER POOL EXECUTION (Requirement #6) ---
    let mut handles = vec![];
    for worker_id in 0..num_workers {
        let rx_clone = Arc::clone(&rx);
        let m_clone = Arc::clone(&metrics);
        
        let handle = thread::spawn(move || {
            loop {
                // Workers block here until a task is available
                let message = rx_clone.lock().unwrap().recv().unwrap();
                
                match message {
                    Some(task) => {
                        let wait_duration = Instant::now().duration_since(task.created_at);
                        
                        println!("Worker {} [EXEC] Task {} ({:?})", worker_id, task.id, task.kind);
                        thread::sleep(task.duration); // Simulate processing
                        
                        let turnaround = Instant::now().duration_since(task.created_at);

                        // Record Metrics
                        m_clone.total_completed.fetch_add(1, Ordering::SeqCst);
                        m_clone.total_wait_time_ms.fetch_add(wait_duration.as_millis() as u64, Ordering::SeqCst);
                        m_clone.total_turnaround_time_ms.fetch_add(turnaround.as_millis() as u64, Ordering::SeqCst);
                    }
                    None => {
                        println!("Worker {} [SHUTDOWN] Signal received.", worker_id);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // --- 5. DISPATCHING (Simulate Arrival Over Time) ---
    for task in tasks {
        tx.send(Some(task)).unwrap();
        // 10ms delay ensures tasks "arrive" rather than teleporting in all at once
        thread::sleep(Duration::from_millis(10)); 
    }

    // --- 6. CLEAN SHUTDOWN (Requirement #8) ---
    for _ in 0..num_workers { tx.send(None).unwrap(); }
    for handle in handles { handle.join().unwrap(); }

    let total_time = system_start.elapsed();

    // --- 7. FINAL REPORTING ---
    let completed = metrics.total_completed.load(Ordering::SeqCst);
    let avg_wait = metrics.total_wait_time_ms.load(Ordering::SeqCst) as f64 / completed as f64;
    let avg_turnaround = metrics.total_turnaround_time_ms.load(Ordering::SeqCst) as f64 / completed as f64;

    let report = format!(
        "\n--- FINAL METRICS REPORT (EXPERIMENT A) --- \n\
        Total Tasks Completed: {}\n\
        Makespan (Total Runtime): {:?}\n\
        Average Wait Time: {:.2} ms\n\
        Average Turnaround Time: {:.2} ms\n\
        ----------------------------\n",
        completed, total_time, avg_wait, avg_turnaround
    );

    println!("{}", report);

    // Auto-save to file for your report
    let mut file = File::create("experiment_results_a.txt").expect("File error");
    file.write_all(report.as_bytes()).expect("Write error");
    println!("Results successfully saved to experiment_results.txt");
}