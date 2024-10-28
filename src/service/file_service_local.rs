use std::fmt::Debug;
use std::path::PathBuf;
use futures_util::future::BoxFuture;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::error::Result;
use crate::service::IFileService;

#[derive(Debug)]
pub struct FileServiceLocal {
    path: PathBuf,
}

impl FileServiceLocal {
    pub fn new(path: &str) -> Self {
        Self { path: PathBuf::from(path) }
    }
}

impl IFileService for FileServiceLocal {
    fn upload(&self, name: String, data: Vec<u8>) -> BoxFuture<Result<()>> {
        let path = self.path.clone();
        Box::pin(async move {
            let path = path.join(name);
            tokio::fs::create_dir_all(&path).await?;
            let mut f = tokio::fs::File::open(&path).await?;
            f.write(&data).await?;
            f.flush().await?;
            Ok(())
        })
    }

    fn download(&self, name: String) -> BoxFuture<Result<Vec<u8>>> {
        let path = self.path.clone();
        Box::pin(async move {
            let path = path.join(name);
            tokio::fs::create_dir_all(&path).await?;
            let mut f = tokio::fs::File::open(&path).await?;
            let mut data = Vec::new();
            f.read_to_end(&mut data).await?;
            Ok(data)
        })
    }

    fn list(&self) -> BoxFuture<Result<Vec<String>>> {
        let path = self.path.clone();
        Box::pin(async move {
            let mut rd = tokio::fs::read_dir(&path).await?;
            let mut result = Vec::new();
            while let Ok(v) = rd.next_entry().await {
                if let Some(v) = v {
                    result.push(v.path().display().to_string());
                }
            }
            Ok(result)
        })
    }
}