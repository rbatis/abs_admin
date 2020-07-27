


use serde::{Deserialize, Serialize};

/// 资源分页DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO{
    pub page: Option<u64>,
    pub size: Option<u64>,
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