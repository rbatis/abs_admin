


use serde::{Deserialize, Serialize};

/// 资源分页DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO{
    pub page: Option<u64>,
    pub size: Option<u64>,
}