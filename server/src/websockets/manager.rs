use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rocket::futures::SinkExt;

pub struct ConnectionManager {
    connections: Arc<Mutex<HashMap<String, ws::stream::DuplexStream>>>,
}

impl ConnectionManager {
    // initializing connection manager
    pub async fn init() -> Self {
        ConnectionManager {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // adding a new connection
    pub async fn add_connection(&mut self, id: String, connection: ws::stream::DuplexStream) {
        self.connections
            .lock()
            .unwrap()
            .insert(id.clone(), connection);
        info!("Added connection with id {:?}", id);
    }

    // dropping a connection
    pub async fn remove_connection(&mut self, id: &str) {
        self.connections.lock().unwrap().remove(id);
        info!("Dropped connection with id {:?}", id);
    }

    // sending a message to connection
    pub async fn send_message(&mut self, id: &str, message: ws::Message) {
        if let Some(connection) = self.connections.lock().unwrap().get_mut(id) {
            let _ = connection.send(message);
        }
    }
}
