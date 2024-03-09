use std::env;

use serde::{Deserialize, Serialize};

// upload response
#[derive(Serialize, Deserialize)]
pub struct UploadResponse {
    pub id: String,
    pub url: String,
}

// status response
#[derive(Serialize, Deserialize)]
pub struct UploadResponseWithStatus {
    pub status_code: u16,
    pub response: UploadResponse,
}

impl UploadResponseWithStatus {
    // success response
    pub fn success(id: String) -> Self {
        UploadResponseWithStatus {
            status_code: 200,
            response: UploadResponse {
                id: id.clone(),
                url: format!(
                    "{}/{}",
                    env::var("BASE_URL").expect("no base url found"),
                    id
                ),
            },
        }
    }
}
