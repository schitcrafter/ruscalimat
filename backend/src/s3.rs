use std::sync::OnceLock;

use aws_credential_types::{credential_fn::provide_credentials_fn, Credentials};
use aws_sdk_s3::{
    config::Region, operation::create_bucket::CreateBucketError, primitives::ByteStream,
    types::CreateBucketConfiguration, Client,
};

use once_cell::sync::Lazy;
use reqwest::StatusCode;
use tracing::info;
use uuid::Uuid;

use crate::config::SETTINGS;

static ACCESS_KEY: Lazy<String> = Lazy::new(|| SETTINGS.get_string("s3.access_key").unwrap());
static SECRET_KEY: Lazy<String> = Lazy::new(|| SETTINGS.get_string("s3.secret_key").unwrap());
static BUCKET_NAME: Lazy<String> = Lazy::new(|| SETTINGS.get_string("s3.bucketname").unwrap());

static S3_CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn init() {
    let region = Region::new(SETTINGS.get_string("s3.region").unwrap());

    let config = aws_config::from_env()
        .endpoint_url(SETTINGS.get_string("s3.url").unwrap())
        .credentials_provider(provide_credentials_fn(credentials_provider))
        .region(region)
        .load()
        .await;

    let s3_config: aws_sdk_s3::Config = (&config).into();
    let s3_config = s3_config.to_builder().force_path_style(true).build();

    let client = aws_sdk_s3::Client::from_conf(s3_config);

    S3_CLIENT.set(client).unwrap();

    create_bucket(BUCKET_NAME.as_str()).await;
}

async fn credentials_provider() -> aws_credential_types::provider::Result {
    Ok(Credentials::from_keys(
        ACCESS_KEY.clone(),
        SECRET_KEY.clone(),
        None,
    ))
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

/// key here has to be a full key, not a part key
pub async fn delete_file(key: &str) -> Result<(), aws_sdk_s3::Error> {
    info!("Deleting file {key}");
    S3_CLIENT
        .get()
        .unwrap()
        .delete_object()
        .bucket(BUCKET_NAME.as_str())
        .key(key)
        .send()
        .await?;

    Ok(())
}

async fn create_bucket(bucket_name: impl Into<String>) {
    let bucket_name = bucket_name.into();
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
                panic!("{:?}", err);
                // panic!("Trying to create bucket went wrong: {err}, {}", err.meta());
            }
        }
    }
}

const VALID_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];

/// This calculates the part of the s3 file key which is independent from
/// the usage, i.e. this has nothing to indicate whether or not it's an
/// account or a product picture.
pub fn partial_picture_key(filename: &str, user_id: &str) -> poem::Result<String> {
    let extension = std::path::Path::new(filename).extension();

    let extension = extension
        .and_then(|ext| ext.to_str())
        .ok_or(poem::Error::from_string(
            "Filename needs to have extension",
            StatusCode::BAD_REQUEST,
        ))?;

    if !VALID_EXTENSIONS.contains(&extension) {
        return Err(poem::Error::from_string(
            format!("Invalid extension {extension}, valid extensions: {VALID_EXTENSIONS:?}"),
            StatusCode::BAD_REQUEST,
        ));
    }

    Ok(format!("{}_{user_id}.{extension}", Uuid::new_v4()))
}

/// turn a generic key into a account-specific one
pub fn full_account_picture_key(part_key: &str) -> String {
    format!("account/{part_key}")
}

/// turn a generic key into a product-specific one
pub fn full_product_picture_key(part_key: &str) -> String {
    format!("product/{part_key}")
}
