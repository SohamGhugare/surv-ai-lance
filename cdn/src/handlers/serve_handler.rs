use std::env;

use rocket::{fs::NamedFile, http::Status, response::status::Custom, serde::json::Json};

use crate::responses::upload_res::{UploadResponse, UploadResponseWithStatus};

#[get("/<id>")]
pub async fn serve(id: &str) -> Result<NamedFile, Custom<Json<UploadResponseWithStatus>>> {
    let fp = format!("./temp/{}.mp4", id);

    match NamedFile::open(fp).await {
        Ok(file) => Ok(file),
        Err(_) => Err(Custom(
            Status::NotFound,
            Json(UploadResponseWithStatus {
                status_code: 404,
                response: UploadResponse {
                    id: id.to_string(),
                    url: format!(
                        "{}/{}",
                        env::var("BASE_URL").expect("no base url found"),
                        id
                    ),
                },
            }),
        )),
    }
}
