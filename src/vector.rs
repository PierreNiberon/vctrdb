use serde::{Deserialize, Serialize};
use std::sync::Mutex;
// Struct that represents a message and its tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub tokens: Vec<u32>,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.tokens == other.tokens
    }
}

// Struct that represents an in-memory vector database
#[derive(Debug)]
pub struct VectorsDb {
    vectors: Mutex<Vec<Message>>, // Vector of messages stored in the database
}

impl VectorsDb {
    // Initializes a new, empty VectorsDb
    pub fn new() -> Self {
        VectorsDb {
            vectors: Mutex::new(Vec::new()),
        }
    }
    // Adds a new message to the database
    pub fn add_message(&self, message: Message) {
        let mut messages = self.vectors.lock().unwrap();
        messages.push(message);
    }
    // Returns a clone of the vector of messages stored in the database
    pub fn get_messages(&self) -> Vec<Message> {
        self.vectors.lock().unwrap().clone()
    }
    // Find a specific message and deletes it
    pub fn delete_message(&self, message: &Message) -> Result<(), String> {
        let mut messages = self.vectors.lock().unwrap();
        if let Some(index) = messages.iter().position(|m| *m == *message) {
            messages.remove(index);
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let db = VectorsDb::new();
        let messages = db.get_messages();
        assert!(messages.is_empty());
    }

    #[test]
    fn test_add_message() {
        let db = VectorsDb::new();
        let message = Message {
            tokens: vec![1, 2, 3],
        };
        db.add_message(message.clone());
        let messages = db.get_messages();
        assert_eq!(messages, vec![message]);
    }

    #[test]
    fn test_get_messages() {
        let db = VectorsDb::new();
        let message1 = Message {
            tokens: vec![1, 2, 3],
        };
        let message2 = Message {
            tokens: vec![4, 5, 6],
        };
        db.add_message(message1.clone());
        db.add_message(message2.clone());
        let messages = db.get_messages();
        assert_eq!(messages, vec![message1, message2]);
    }

    #[test]
    fn test_delete_message() {
        let db = VectorsDb::new();
        let message = Message {
            tokens: vec![1, 2, 3],
        };
        db.add_message(message.clone());
        db.delete_message(&message).unwrap();
        let messages = db.get_messages();
        assert!(messages.is_empty());
    }
}
