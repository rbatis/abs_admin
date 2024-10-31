use std::fmt::Debug;
use futures_util::future::BoxFuture;
use crate::error::Result;

pub trait IFileService: Sync + Send + Debug {
    fn upload(&self, name: String, data: Vec<u8>) -> BoxFuture<Result<()>>;
    fn download(&self, name: String) -> BoxFuture<Result<Vec<u8>>>;
    fn list(&self,name:String) -> BoxFuture<Result<Vec<String>>>;
    fn remove(&self, name: String) -> BoxFuture<Result<()>>;
}