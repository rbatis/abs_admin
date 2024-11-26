use crate::error::{Error, Result};
use crate::service::FileLocalService;
use futures_util::future::BoxFuture;
use std::fmt::Debug;
use std::ops::Deref;

pub trait IStorageService: Sync + Send + Debug {
    fn upload(&self, name: String, data: Vec<u8>) -> BoxFuture<Result<String>>;
    fn download(&self, name: String) -> BoxFuture<Result<Vec<u8>>>;
    fn list(&self, name: String) -> BoxFuture<Result<Vec<String>>>;
    fn remove(&self, name: String) -> BoxFuture<Result<()>>;
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
