use std::path::Path;

use rocket::{form::Form, http::Status, response::status::Custom, serde::json::Json};

use crate::{
    models::upload::UploadForm, responses::upload_res::UploadResponse, utils::keygen::generate_key,
};

#[post("/upload", data = "<upload>")]
pub async fn upload(mut upload: Form<UploadForm<'_>>) -> Custom<Json<UploadResponse>> {
    let id = generate_key(6);
    let fp = format!("./temp/{}.avi", id.clone());
    upload
        .file
        .persist_to(fp)
        .await
        .expect("failed to save file");

    Custom(Status::Ok, Json(UploadResponse::new(id)))
}
