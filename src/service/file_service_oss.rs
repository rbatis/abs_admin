use std::path::PathBuf;
use aws_config::SdkConfig;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use futures_util::future::BoxFuture;
use tokio::io::AsyncReadExt;
use crate::error::Error;
use crate::service::{IFileService};

#[derive(Debug)]
pub struct FileServiceOss {
    path: PathBuf,
    client: Client,
    bucket: String,
}

impl FileServiceOss {
    pub fn new(path: &str) -> Self {
        //TODO
        let shared_config = SdkConfig::builder()
            .build();
        let client = Client::new(&shared_config);
        Self {
            path: PathBuf::from(path),
            client: client,
            bucket: "".to_string(),
        }
    }
}

impl IFileService for FileServiceOss {
    fn upload(&self, name: String, data: Vec<u8>) -> BoxFuture<crate::error::Result<()>> {
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