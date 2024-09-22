use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UploadRequest {
    pub file_name: String,
    pub file_type: String,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub uuid: String,
    pub url: String,
}
