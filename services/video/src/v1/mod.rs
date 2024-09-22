use axum::Router;
use video_on_demand_core::s3::S3Client;

use crate::v1::routes::create_upload_routes;

pub mod routes;

pub fn create_v1_routes() -> Router<S3Client> {
    Router::new().nest("/upload", create_upload_routes())
}
