use std::{env, sync::OnceLock};

use aws_sdk_s3::{
    config::Region, operation::create_bucket::CreateBucketError, primitives::ByteStream,
    types::CreateBucketConfiguration, Client,
};
use once_cell::sync::Lazy;
use tracing::info;

static BUCKET_NAME: Lazy<String> = Lazy::new(|| env::var("S3_BUCKETNAME").unwrap());
static S3_REGION: Lazy<String> = Lazy::new(|| env::var("S3_REGION").unwrap());

static S3_CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn init() {
    let config = aws_config::from_env()
        .endpoint_url(env::var("S3_URL").unwrap())
        .region(Region::new(S3_REGION.as_str()))
        .load()
        .await;

    let s3_config: aws_sdk_s3::Config = (&config).into();
    let s3_config = s3_config.to_builder().force_path_style(true).build();

    let client = aws_sdk_s3::Client::from_conf(s3_config);

    S3_CLIENT.set(client).unwrap();

    create_bucket(BUCKET_NAME.as_str()).await;
}

pub async fn upload_file(key: &str, body: ByteStream) -> Result<(), aws_sdk_s3::Error> {
    info!("Uploading file {key}");
    S3_CLIENT
        .get()
        .unwrap()
        .put_object()
        .bucket(BUCKET_NAME.as_str())
        .key(key)
        .body(body)
        .send()
        .await?;

    Ok(())
}

async fn create_bucket(bucket_name: &str) {
    info!("Trying to create bucket {bucket_name}");
    let cfg = CreateBucketConfiguration::builder().build();

    let result = S3_CLIENT
        .get()
        .unwrap()
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await;

    if let Err(err) = result {
        match err.into_service_error() {
            CreateBucketError::BucketAlreadyExists(_)
            | CreateBucketError::BucketAlreadyOwnedByYou(_) => {}
            err => {
                Err::<(), _>(err).unwrap();
                // panic!("Trying to create bucket went wrong: {err}, {}", err.meta());
            }
        }
    }
}
