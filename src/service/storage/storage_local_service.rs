use crate::error::Result;
use crate::service::IStorageService;
use std::fmt::Debug;
use std::path::PathBuf;
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct FileLocalService {}

impl FileLocalService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IStorageService for FileLocalService {
    async fn upload(&self, name: String, data: Vec<u8>) -> Result<String> {
        let name = PathBuf::from(name);
        if let Some(parent) = name.parent() {
            tokio::fs::create_dir_all(&parent).await?;
        }
        let mut f = tokio::fs::File::create(&name).await?;
        f.write(&data).await?;
        f.flush().await?;
        Ok(name.to_str().unwrap_or_default().to_string())
    }

    async fn download(&self, name: String) -> Result<Vec<u8>> {
        let name = PathBuf::from(name);
        if let Some(parent) = name.parent() {
            tokio::fs::create_dir_all(&parent).await?;
        }
        let mut f = tokio::fs::File::open(&name).await?;
        let mut data = Vec::new();
        f.read_to_end(&mut data).await?;
        Ok(data)
    }

    async fn list(&self, name: String) -> Result<Vec<String>> {
        let name = PathBuf::from(name);
        let mut rd = tokio::fs::read_dir(&name).await?;
        let mut result = Vec::new();
        while let Ok(v) = rd.next_entry().await {
            if let Some(v) = v {
                result.push(v.path().display().to_string());
            } else {
                break;
            }
        }
        Ok(result)
    }

    async  fn remove(&self, name: String) -> Result<()> {
        let name = PathBuf::from(name);
        let f = tokio::fs::remove_file(&name).await?;
        Ok(f)
    }
}
