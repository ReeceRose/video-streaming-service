use std::env;

use async_nats::jetstream::{self, consumer::PullConsumer};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.0:4222".to_string());

    let client = async_nats::connect(nats_url)
        .await
        .expect("NATS: Failed to connect to NATS server");

    let jetstream = jetstream::new(client);

    let stream_name = String::from("EVENTS");

    let consumer: PullConsumer = jetstream
        .create_stream(jetstream::stream::Config {
            name: stream_name,
            subjects: vec!["events.>".to_string()],
            ..Default::default()
        })
        .await
        .expect("NATS: Failed to create stream")
        .create_consumer(jetstream::consumer::pull::Config {
            durable_name: Some("consumer".to_string()),
            ..Default::default()
        })
        .await
        .expect("NATS: Failed to create consumer");
    let mut messages = consumer
        .messages()
        .await
        .expect("NATS: Failed to get messages");

    while let Some(message) = messages.next().await {
        let message = message.expect("NATS: Failed to decode? message");
        message
            .ack()
            .await
            .expect("NATS: Failed to acknowledge message");

        println!(
            "Received message of: '{}' on '{}'",
            String::from_utf8_lossy(&message.payload),
            String::from_utf8_lossy(message.subject.as_bytes())
        );
    }
}
