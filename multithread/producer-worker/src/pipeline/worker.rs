use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub enum Message {
    Job(String),
    Shutdown,
}

pub struct WorkerHandle {
    join: Option<JoinHandle<()>>,
}

impl WorkerHandle {
    pub fn start(rx: Receiver<Message>) -> Self {
        let join = thread::spawn(move || {
            println!("[worker] started");
            while let Ok(msg) = rx.recv() {
                match msg {
                    Message::Job(job) => {
                        println!("[worker] processing: {}", job);
                        thread::sleep(Duration::from_millis(500));
                    }
                    Message::Shutdown => {
                        println!("[worker] graceful shutdown");
                        break;
                    }
                };
            }
            println!("[worker] exiting");
        });
        Self { join: Some(join) }
    }

    pub fn join(mut self) {
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

impl Drop for WorkerHandle {
    fn drop(&mut self) {
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}
