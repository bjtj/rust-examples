mod pipeline;

use pipeline::{Producer, WorkerHandle};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let worker = WorkerHandle::start(rx);
    let producer = Producer::new(tx.clone());
    for i in 0..5 {
        producer.send_job(format!("job-{}", i));
    }

    let p2 = producer.clone();
    thread::spawn(move || {
        for i in 100..103 {
            p2.send_job(format!("job-{}", i));
            thread::sleep(Duration::from_millis(200));
        }
    });
    thread::sleep(Duration::from_secs(3));
    producer.shutdown_worker();
    worker.join();
    println!("[main] done");
}
