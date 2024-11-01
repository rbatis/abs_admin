mod storage_service_local;
#[cfg(feature = "storage_s3")]
mod storage_service_oss;

pub use storage_service_local::*;
#[cfg(feature = "storage_s3")]
pub use storage_service_oss::*;