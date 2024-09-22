use axum::{routing::post, Router};
use upload::generate_presigned_url;
use video_on_demand_core::s3::S3Client;

pub mod upload;

pub fn create_upload_routes() -> Router<S3Client> {
    Router::new().route("/", post(generate_presigned_url))
}
