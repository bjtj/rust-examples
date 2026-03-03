use std::sync::mpsc::Sender;

use super::worker::Message;

#[derive(Clone)]
pub struct Producer {
    tx: Sender<Message>,
}

impl Producer {
    pub fn new(tx: Sender<Message>) -> Self {
        Self { tx }
    }

    pub fn send_job(&self, job: String) {
        let _ = self.tx.send(Message::Job(job));
    }

    pub fn shutdown_worker(&self) {
        let _ = self.tx.send(Message::Shutdown);
    }
}
