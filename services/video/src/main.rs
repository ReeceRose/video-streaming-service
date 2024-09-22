mod models;
mod v1;

use crate::v1::create_v1_routes;
use axum::Router;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::prelude::*;
use video_on_demand_core::s3::S3Client;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "video=debug,axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let client = S3Client::new().await;

    let app = Router::new()
        .nest("/v1", create_v1_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
