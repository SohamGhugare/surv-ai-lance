use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use ws::Message;

use super::guard::ConnectionManagerGuard;

#[get("/ws")]
pub fn websocket_channel(ws: ws::WebSocket, state: ConnectionManagerGuard) -> ws::Channel<'_> {
    use rocket::futures::{SinkExt, StreamExt};

    // let connection_manager = state.inner();

    ws.channel(move |stream| {
        Box::pin(async move {
            let connection_manager = state.0;
            let ws_stream = Arc::new(TokioMutex::new(stream));
            let mut stream = ws_stream.lock().await;
            if let Some(id) = stream.next().await {
                if let Ok(id) = id {
                    info!("Client connected with id {:?}", id);

                    let mut manager = connection_manager.lock().await;
                    manager
                        .add_connection(id.to_string(), ws_stream.clone())
                        .await;
                }
            }
            while let Some(message) = stream.next().await {
                if let Ok(msg) = message {
                    info!("Received message: {:?}", msg);
                }
                let _ = stream.send(Message::from("Received!")).await;
            }
            Ok(())
        })
    })
}
