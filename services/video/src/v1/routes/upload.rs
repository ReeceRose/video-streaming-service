use axum::{extract::State, response::IntoResponse, Json};
use uuid::Uuid;
use video_on_demand_core::s3::S3Client;

use crate::models::upload::{UploadRequest, UploadResponse};

pub async fn generate_presigned_url(
    State(s3_client): State<S3Client>,
    Json(request): Json<UploadRequest>,
) -> impl IntoResponse {
    let uuid = String::from(Uuid::new_v4());

    let url = s3_client
        .generate_put_object_presigned_url(
            String::from("videos"),
            uuid.to_string(),
            request.file_type,
        )
        .await;

    let response = UploadResponse { url, uuid };

    Json(response)
}
