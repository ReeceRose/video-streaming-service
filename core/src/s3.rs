use core::time;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, SdkConfig};
use aws_sdk_s3::{presigning::PresigningConfig, Client};

#[derive(Clone)]
pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new() -> Self {
        S3Client {
            client: create_s3_client().await,
        }
    }

    pub async fn generate_put_object_presigned_url(
        &self,
        bucket: String,
        file_name: String,
        content_type: String,
    ) -> String {
        let url = self
            .client
            .put_object()
            .bucket(bucket)
            .key(file_name)
            .content_type(content_type) // Specify Content-Type
            .presigned(
                PresigningConfig::expires_in(time::Duration::from_secs(100))
                    .expect("AWS: Failed to get presigned_request"),
            )
            .await
            .expect("AWS: Failed to get presigned URL");

        String::from(url.uri())
    }
}

async fn load_aws_config() -> SdkConfig {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    return aws_config::from_env().region(region_provider).load().await;
}

async fn create_s3_client() -> Client {
    let config = load_aws_config().await;

    return Client::from_conf(
        aws_sdk_s3::Config::builder()
            .region(config.region().cloned())
            .endpoint_url("http://127.0.0.1:4566")
            .behavior_version(BehaviorVersion::v2024_03_28())
            .build(),
    );
}
