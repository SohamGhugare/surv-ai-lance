use ws::Message;

#[get("/ws")]
pub fn websocket_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| {
        Box::pin(async move {
            if let Some(id) = stream.next().await {
                if let Ok(id) = id {
                    info!("Client connected with id {:?}", id);
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
