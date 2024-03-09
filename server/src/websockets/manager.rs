use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex as TokioMutex;

use rocket::futures::SinkExt;

pub struct ConnectionManager {
    pub connections: HashMap<String, Arc<TokioMutex<ws::stream::DuplexStream>>>,
}

impl ConnectionManager {
    // initializing connection manager
    pub async fn init() -> Self {
        ConnectionManager {
            connections: HashMap::new(),
        }
    }

    // adding a new connection
    pub async fn add_connection(
        &mut self,
        id: String,
        connection: Arc<TokioMutex<ws::stream::DuplexStream>>,
    ) {
        self.connections.insert(id.clone(), connection);
        info!("Added connection with id {:?}", id);
    }

    // dropping a connection
    pub async fn _remove_connection(&mut self, id: &str) {
        self.connections.remove(id);
        info!("Dropped connection with id {:?}", id);
    }

    // sending a message to connection
    pub async fn send_message(&mut self, id: &str, message: ws::Message) {
        info!("Sending...");
        if let Some(connection) = self.connections.get_mut(id) {
            let _ = connection.lock().await.send(message);
        }
        info!("Sent message to connection with id {:?}", id);
    }
}
