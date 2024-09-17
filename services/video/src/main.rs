use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "video=debug,axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<Value> {
    Json(json!({ "msg": "Hello, world!" }))
}
