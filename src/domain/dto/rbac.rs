use crate::domain::table::rbac::{RbacPermission, RbacRole, RbacUserRole};
use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::PageRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PermissionPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

impl From<&PermissionPageDTO> for PageRequest {
    fn from(arg: &PermissionPageDTO) -> Self {
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

impl From<PermissionAddDTO> for RbacPermission {
    fn from(arg: PermissionAddDTO) -> Self {
        RbacPermission {
            id: ObjectId::new().to_string().into(),
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

impl From<&ResEditDTO> for RbacPermission {
    fn from(arg: &ResEditDTO) -> Self {
        RbacPermission {
            id: arg.id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            create_date: None,
        }
    }
}


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

impl From<RoleAddDTO> for RbacRole {
    fn from(arg: RoleAddDTO) -> Self {
        RbacRole {
            id: ObjectId::new().to_string().into(),
            name: arg.name,
            create_date: DateTime::now().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub permission_ids: Vec<String>,
}

impl From<RoleEditDTO> for RbacRole {
    fn from(arg: RoleEditDTO) -> Self {
        RbacRole {
            id: arg.id,
            name: arg.name,
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
    pub permission_ids: Vec<String>,
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
    pub permission_ids: Vec<String>,
}

impl From<SysRoleResUpdateDTO> for RoleEditDTO {
    fn from(arg: SysRoleResUpdateDTO) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            permission_ids: arg.permission_ids,
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


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleAddDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

impl From<UserRoleAddDTO> for RbacUserRole {
    fn from(arg: UserRoleAddDTO) -> Self {
        RbacUserRole {
            id: arg.id.clone(),
            user_id: arg.user_id.clone(),
            role_id: arg.role_id.clone(),
            create_date: DateTime::now().into(),
        }
    }
}