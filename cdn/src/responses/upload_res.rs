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
