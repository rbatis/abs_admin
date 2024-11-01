use std::fmt::Debug;
use std::ops::Deref;
use futures_util::future::BoxFuture;
use crate::error::Result;
use crate::service::{FileServiceLocal, FileServiceOss, S3Config};

pub trait IStorageService: Sync + Send + Debug {
    fn upload(&self, name: String, data: Vec<u8>) -> BoxFuture<Result<()>>;
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
            Self {
                inner: Box::new(FileServiceLocal::new(storage))
            }
        } else if storage.starts_with("s3://") {
            Self {
                inner: Box::new(FileServiceOss::new(storage, S3Config::load(storage).unwrap()))
            }
        } else {
            panic!("warn config of storage url={}", storage)
        }
    }
}