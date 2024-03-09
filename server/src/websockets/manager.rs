use std::{collections::HashMap, sync::Arc};

use rocket::futures::SinkExt;
use tokio::sync::Mutex as TokioMutex;

pub struct ConnectionManager {
    pub connections: Arc<TokioMutex<HashMap<String, ws::stream::DuplexStream>>>,
    // pub connections: HashMap<String, Arc<TokioMutex<ws::stream::DuplexStream>>>,
}

impl ConnectionManager {
    // initializing connection manager
    pub async fn init() -> Self {
        ConnectionManager {
            connections: Arc::new(TokioMutex::new(HashMap::new())),
        }
    }

    // adding a new connection
    pub async fn add_connection(&mut self, id: String, connection: ws::stream::DuplexStream) {
        self.connections.lock().await.insert(id.clone(), connection);
        info!("Added connection with id {:?}", id);
    }

    // sending a message to connection
    pub async fn send_message(&self, id: String, message: ws::Message) {
        // let id = id.to_owned();
        info!("Sending...");
        if let Some(stream) = self.connections.lock().await.get_mut(&id) {
            match stream.send(message.clone()).await {
                Ok(_) => info!("Sent message: {:?}", message),
                Err(e) => error!("Failed to send message: {:?}", e),
            }
        } else {
            info!("No connection found with id {:?}", id);
            // tokio::spawn(async {})
        }
    }
}
