use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng as _; 

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let shared_rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // TODO: Create 2 producer threads
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let h = thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT / NUM_PRODUCERS);
        });
        handles.push(h);
    }

    // TODO: Create 3 consumer threads
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&shared_rx);
        let h = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        handles.push(h);
    }

    // After producers finish, send termination signal for each consumer
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    
    // TODO: Generate random numbers and send them to the channel
    for _ in 0..item_count {
        let val = rng.gen_range(1..100);
        println!("[P{}] generated: {}", id, val);
        tx.send(val).ok();
        thread::sleep(Duration::from_millis(75));
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    loop {
        let msg = rx.lock().unwrap().recv().unwrap();

        if msg == TERMINATION_SIGNAL {
            println!("[C{}] received shutdown.", id);
            break;
        }

        println!("[C{}] processed: {}", id, msg);
        thread::sleep(Duration::from_millis(100));
    }
}