use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Message {
    tokens: Vec<u32>,
}

#[derive(Default)]
pub struct VectorsDb {
    vectors: Arc<Mutex<Vec<Message>>>,
}

impl VectorsDb {
    pub fn add_message(&self, message: Message) {
        let mut messages = self.messages.lock().unwrap();
        messages.push(message);
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.lock().unwrap().clone()
    }
}
