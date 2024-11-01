use std::fmt::Debug;
use std::ops::Deref;
use futures_util::future::BoxFuture;
use crate::error::Result;
use crate::service::{FileLocalService};

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
    pub fn new(storage: &str) -> StorageService {
        if storage == "local" {
            return Self {
                inner: Box::new(FileLocalService::new())
            };
        } else if storage.starts_with("s3://") {
            #[cfg(feature = "storage_s3")]
            {
                return Self {
                    inner: Box::new(crate::service::FileS3Service::new(crate::service::S3Config::load(storage).unwrap()))
                };
            }
        }
        panic!("unknown storage url={}. did you forget open default `[features]` on Cargo.toml?", storage)
    }
}