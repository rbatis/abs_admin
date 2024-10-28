use std::fmt::Debug;
use futures_util::future::BoxFuture;
use crate::error::Result;

pub trait IFileService: Sync + Send + Debug {
    fn upload(&self, name: &str, data: Vec<u8>) -> BoxFuture<Result<()>>;
    fn download(&self, name: &str) -> BoxFuture<Result<Vec<u8>>>;
    fn list(&self) -> BoxFuture<Result<Vec<String>>>;
}