


use serde::{Deserialize, Serialize};

/// 资源分页DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO{
    pub page: Option<u64>,
    pub size: Option<u64>,
}

/// 资源添加DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResAddDTO{
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}


/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO{
    pub account: Option<String>,
    pub password: Option<String>,
}

/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO{
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>
}

/// 用户分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO{
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>
}

/// 角色分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolePageDTO{
    pub page: Option<u64>,
    pub size: Option<u64>
}