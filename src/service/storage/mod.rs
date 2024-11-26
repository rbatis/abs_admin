mod storage_local_service;
#[cfg(feature = "storage_s3")]
mod storage_oss_service;

pub use storage_local_service::*;
#[cfg(feature = "storage_s3")]
pub use storage_oss_service::*;
