use std::fmt::Debug;
use crate::error::Result;

pub trait IFileService: Sync + Send + Debug {
    fn upload(&self, name: &str, data: Vec<u8>) -> Result<()>;
    fn download(&self, name: &str) -> Result<Vec<u8>>;
    fn list(&self) -> Result<Vec<String>>;
}