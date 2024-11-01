use std::path::PathBuf;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, Config};
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use crate::error::Error;
use crate::service::{IFileService};

#[derive(Debug)]
pub struct FileServiceOss {
    path: PathBuf,
    client: Client,
    bucket: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct S3Config {
    pub bucket: String,
    pub endpoint_url: String,
    pub access_key: String,
    pub secret_key: String,
}

impl FileServiceOss {
    pub fn new(path: &str, cfg: S3Config) -> Self {
        let credentials = Credentials::new(cfg.access_key, cfg.secret_key, None, None, "minio");
        let config = Config::builder()
            .region(Region::from_static("us-east-1")) // MinIO 可以使用任何 Region 值
            .credentials_provider(credentials)
            .endpoint_url(cfg.endpoint_url)
            .behavior_version(BehaviorVersion::latest())
            .build();
        let client = Client::from_conf(config);
        Self {
            path: PathBuf::from(path),
            client: client,
            bucket: cfg.bucket.to_string(),
        }
    }
}

impl IFileService for FileServiceOss {
    fn upload(&self, name: String, data: Vec<u8>) -> BoxFuture<crate::error::Result<()>> {
        let name = name.trim_start_matches("/").to_string();
        let path = self.path.clone();
        let name = path.join(name);
        Box::pin(async move {
            let _resp = self.client.put_object()
                .bucket(&self.bucket)
                .key(name.to_str().unwrap_or_default())
                .body(ByteStream::from(data))
                .send().await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(())
        })
    }

    fn download(&self, name: String) -> BoxFuture<crate::error::Result<Vec<u8>>> {
        let name = name.trim_start_matches("/").to_string();
        let path = self.path.clone();
        let name = path.join(name);
        Box::pin(async move {
            let resp = self.client.get_object()
                .bucket(&self.bucket)
                .key(name.to_str().unwrap_or_default())
                .send().await
                .map_err(|e| Error::from(e.to_string()))?;
            let mut buf = vec![];
            resp.body.into_async_read().read_to_end(&mut buf).await?;
            Ok(buf)
        })
    }

    fn list(&self, name: String) -> BoxFuture<crate::error::Result<Vec<String>>> {
        let name = name.trim_start_matches("/").to_string();
        let path = self.path.clone();
        let name = path.join(name);
        Box::pin(async move {
            let resp = self.client.list_objects_v2()
                .bucket(&self.bucket)
                .prefix(name.to_str().unwrap_or_default())
                .send().await
                .map_err(|e| Error::from(e.to_string()))?;
            let mut data = vec![];
            for object in resp.contents() {
                println!("{}", object.key().unwrap_or_default());
                data.push(object.key().unwrap_or_default().to_string());
            }
            Ok(data)
        })
    }

    fn remove(&self, name: String) -> BoxFuture<crate::error::Result<()>> {
        let name = name.trim_start_matches("/").to_string();
        let path = self.path.clone();
        let name = path.join(name);
        Box::pin(async move {
            let _resp = self.client.delete_object()
                .bucket(&self.bucket)
                .key(name.to_str().unwrap_or_default())
                .send().await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(())
        })
    }
}