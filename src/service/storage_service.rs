use crate::error::{Error, Result};
use crate::service::FileLocalService;
use std::fmt::Debug;
use std::ops::Deref;
use async_trait::async_trait;

#[async_trait]
pub trait IStorageService: Sync + Send + Debug {
    async fn upload(&self, name: String, data: Vec<u8>) -> Result<String>;
    async fn download(&self, name: String) -> Result<Vec<u8>>;
    async fn list(&self, name: String) -> Result<Vec<String>>;
    async fn remove(&self, name: String) -> Result<()>;
}

pub struct StorageService {
    pub inner: Box<dyn IStorageService>,
}
impl Deref for StorageService {
    type Target = dyn IStorageService;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl StorageService {
    pub fn new(storage: &str) -> Result<StorageService> {
        if storage == "local" {
            return Ok(Self {
                inner: Box::new(FileLocalService::new()),
            });
        } else if storage.starts_with("s3://") {
            #[cfg(feature = "storage_s3")]
            {
                return Ok(Self {
                    inner: Box::new(crate::service::FileS3Service::new(
                        crate::service::S3Config::load(storage)?,
                    )),
                });
            }
        }
        Err(Error::from(format!(
            "Unsupported storage service: {}",
            storage
        )))
    }
}
