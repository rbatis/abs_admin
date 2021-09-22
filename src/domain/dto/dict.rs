use serde::{Deserialize, Serialize};

/// 字典分页DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}

/// 字典分添加DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictAddDTO {
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}

/// 字典修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
}
