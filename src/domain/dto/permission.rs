use crate::domain::table::SysPermission;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl From<&ResPageDTO> for PageRequest {
    fn from(arg: &ResPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PermissionAddDTO {
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

impl From<PermissionAddDTO> for SysPermission {
    fn from(arg: PermissionAddDTO) -> Self {
        SysPermission {
            id: ObjectId::new().to_string().into(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            create_date: Some(DateTime::now()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResEditDTO {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

impl From<&ResEditDTO> for SysPermission {
    fn from(arg: &ResEditDTO) -> Self {
        SysPermission {
            id: arg.id.clone(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            create_date: None,
        }
    }
}
