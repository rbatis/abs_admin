
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO{
    pub page: Option<u64>,
    pub size: Option<u64>,
}