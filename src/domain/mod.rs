use rbatis::crud::CRUDEnable;
use serde::{Deserialize, Serialize};

///权限资源表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BizResource {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<String>,
}

impl CRUDEnable for BizResource {
    type IdType = String;
}