use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use super::manager::ConnectionManager;

pub struct ConnectionManagerGuard<'a>(pub &'a Arc<TokioMutex<ConnectionManager>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectionManagerGuard<'r> {
    type Error = std::io::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let connection_manager = req.rocket().state::<Arc<TokioMutex<ConnectionManager>>>();

        match connection_manager {
            Some(cm) => Outcome::Success(ConnectionManagerGuard(cm)),
            None => Outcome::Forward(rocket::http::Status::ServiceUnavailable),
        }
    }
}
