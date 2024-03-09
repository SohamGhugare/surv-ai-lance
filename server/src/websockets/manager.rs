use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rocket::futures::SinkExt;

pub struct ConnectionManager {
    connections: HashMap<String, Arc<Mutex<ws::stream::DuplexStream>>>,
}

impl ConnectionManager {
    // initializing connection manager
    pub async fn init() -> Self {
        ConnectionManager {
            connections: HashMap::new(),
        }
    }

    // adding a new connection
    pub async fn add_connection(&mut self, id: String, connection: Arc<Mutex<ws::stream::DuplexStream>>>) {
        self.connections.insert(id.clone(), connection);
        info!("Added connection with id {:?}", id);
    }

    // dropping a connection
    pub async fn remove_connection(&mut self, id: &str) {
        self.connections.remove(id);
        info!("Dropped connection with id {:?}", id);
    }

    // sending a message to connection
    pub async fn send_message(&mut self, id: &str, message: ws::Message) {
        if let Some(connection) = self.connections.get_mut(id) {
            let _ = connection.lock().unwrap().send(message);
        }
    }
}
