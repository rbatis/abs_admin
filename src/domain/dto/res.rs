use serde::{Deserialize, Serialize};

/// 资源分页DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

/// 资源添加DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResAddDTO {
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

/// 资源修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResEditDTO {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}
