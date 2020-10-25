use serde::{Deserialize, Serialize};

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}

/// 资源分页DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
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


/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub account: Option<String>,
    pub password: Option<String>,
}

/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

/// 用户修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserEditDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

/// 用户分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,
}

/// 角色分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolePageDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
}

/// 角色添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleAddDTO {
    pub name: Option<String>,
    pub auths: Vec<String>,
    //父id(可空)
    pub parent_id:Option<String>,
}

/// 角色修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub auths: Vec<String>,
    pub parent_id:Option<String>,
}