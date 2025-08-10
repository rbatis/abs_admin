use crate::error::Error;
use crate::service::IStorageService;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, Config};
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct FileS3Service {
    client: Client,
    bucket: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct S3Config {
    pub bucket: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

impl S3Config {
    pub fn load(arg: &str) -> Result<S3Config, Error> {
        if arg.starts_with("s3://") {
            let v = serde_json::from_str(arg.trim_start_matches("s3://"))
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(v)
        } else {
            Err(Error::from("s3 must have prefix 's3://'"))
        }
    }
}

impl FileS3Service {
    pub fn new(cfg: S3Config) -> Self {
        let credentials = Credentials::new(cfg.access_key, cfg.secret_key, None, None, "minio");
        let config = Config::builder()
            .region(Region::new({
                if cfg.region.is_empty() {
                    Cow::Borrowed("us-east-1")
                } else {
                    Cow::Owned(cfg.region)
                }
            }))
            .credentials_provider(credentials)
            .endpoint_url(cfg.endpoint)
            .behavior_version(BehaviorVersion::latest())
            .build();
        let client = Client::from_conf(config);
        Self {
            client: client,
            bucket: cfg.bucket.to_string(),
        }
    }
}

#[async_trait]
impl IStorageService for FileS3Service {
    async fn upload(&self, name: String, data: Vec<u8>) -> crate::error::Result<String> {
        let name = name.trim_start_matches("/").to_string();
        let name = PathBuf::from(name);
        let _resp = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(name.to_str().unwrap_or_default())
            .body(ByteStream::from(data))
            .send()
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        Ok(name.to_str().unwrap_or_default().to_string())
    }

    async  fn download(&self, name: String) -> crate::error::Result<Vec<u8>> {
        let name = name.trim_start_matches("/").to_string();
        let name = PathBuf::from(name);
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(name.to_str().unwrap_or_default())
            .send()
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let mut buf = vec![];
        resp.body.into_async_read().read_to_end(&mut buf).await?;
        Ok(buf)
    }

    async fn list(&self, name: String) -> crate::error::Result<Vec<String>> {
        let name = name.trim_start_matches("/").to_string();
        let name = PathBuf::from(name);
        let resp = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(name.to_str().unwrap_or_default())
            .send()
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let mut data = vec![];
        for object in resp.contents() {
            data.push(object.key().unwrap_or_default().to_string());
        }
        Ok(data)
    }

    async fn remove(&self, name: String) -> crate::error::Result<()> {
        let name = name.trim_start_matches("/").to_string();
        let name = PathBuf::from(name);
        let _resp = self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(name.to_str().unwrap_or_default())
            .send()
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        Ok(())
    }
}
