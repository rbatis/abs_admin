use serde::{Deserialize, Serialize};

/// 角色分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

/// 角色添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleAddDTO {
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
}

/// 角色修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub resource_ids: Vec<String>,
}

/// 角色资源添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResAddDTO {
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    //资源id集合
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

/// 角色资源添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResUpdateDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    //资源id集合
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

/// 角色资源分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}
