use crate::domain::table::SysRole;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl From<&RolePageDTO> for PageRequest {
    fn from(arg: &RolePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleAddDTO {
    pub name: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
}

impl From<RoleAddDTO> for SysRole {
    fn from(arg: RoleAddDTO) -> Self {
        SysRole {
            id: ObjectId::new().to_string().into(),
            name: arg.name,
            parent_id: arg.parent_id,
            create_date: DateTime::now().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub resource_ids: Vec<String>,
}

impl From<RoleEditDTO> for SysRole {
    fn from(arg: RoleEditDTO) -> Self {
        SysRole {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            create_date: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResAddDTO {
    pub name: Option<String>,
    //father id(可空)
    pub parent_id: Option<String>,
    //resource id vec
    pub resource_ids: Vec<String>,
}

impl From<SysRoleResAddDTO> for RoleAddDTO {
    fn from(arg: SysRoleResAddDTO) -> Self {
        Self {
            name: arg.name,
            parent_id: arg.parent_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResUpdateDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    //resource id vec
    pub resource_ids: Vec<String>,
}

impl From<SysRoleResUpdateDTO> for RoleEditDTO {
    fn from(arg: SysRoleResUpdateDTO) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            resource_ids: arg.resource_ids,
            parent_id: arg.parent_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl From<&SysRoleResPageDTO> for PageRequest {
    fn from(arg: &SysRoleResPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
