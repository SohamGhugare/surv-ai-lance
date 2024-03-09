use std::sync::Arc;

use rocket::{futures::SinkExt, http::Status, response::status::Custom, serde::json::Json};
use ws::Message;

use crate::{
    models::case::{Case, CreateCase},
    websockets::guard::ConnectionManagerGuard,
};

#[post("/new", format = "json", data = "<case>")]
pub async fn new_case_handler(case: Json<CreateCase>) -> Custom<Json<Case>> {
    let case = Case::new(case.into_inner());
    Custom(Status::Ok, Json(case))
}

#[post("/broadcast", format = "json", data = "<msg>")]
pub async fn broadcast_handler(
    msg: Json<String>,
    state: ConnectionManagerGuard<'_>,
) -> Custom<Json<String>> {
    let conn_manager = state.0;
    let manager = conn_manager.lock().await;
    let connections: Vec<_> = manager.connections.lock().await.keys().cloned().collect();

    for id in connections {
        info!("Broadcasting {:?} to {:?}", msg.to_string(), id);
        let _res = manager
            .send_message(id, Message::from(msg.to_string()))
            .await;
    }

    Custom(Status::Ok, Json(String::from("Broadcasted!")))
}
