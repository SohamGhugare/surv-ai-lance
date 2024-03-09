use std::sync::Arc;

use super::guard::ConnectionManagerGuard;

#[get("/ws")]
pub fn websocket_channel(ws: ws::WebSocket, state: ConnectionManagerGuard) -> ws::Channel<'_> {
    use rocket::futures::StreamExt;

    // let connection_manager = state.inner();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let connection_manager = Arc::clone(state.0);

            if let Some(id) = stream.next().await {
                if let Ok(id) = id {
                    info!("Client connected with id {:?}", id);

                    let mut manager = connection_manager.lock().await;
                    manager.add_connection(id.to_string(), stream).await;
                }
            }

            Ok(())
        })
    })
}
