use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};
use crate::domain::table::SysRes;

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
pub struct ResAddDTO {
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

impl From<ResAddDTO> for SysRes{
    fn from(arg: ResAddDTO) -> Self {
        SysRes {
            id: ObjectId::new().to_string().into(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            del: 0.into(),
            create_date: FastDateTime::now().set_micro(0).into(),
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

impl From<&ResEditDTO> for SysRes{
    fn from(arg: &ResEditDTO) -> Self {
        SysRes {
            id: arg.id.clone(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            del: None,
            create_date: None,
        }
    }
}
