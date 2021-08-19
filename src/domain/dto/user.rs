use crate::domain::domain::LoginCheck;
use serde::{Deserialize, Serialize};

/// 用户
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
}

/// 用户修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserEditDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub state: Option<i32>,
    pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
}

/// 用户分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,
}

impl From<&UserRolePageDTO> for UserPageDTO {
    fn from(arg: &UserRolePageDTO) -> Self {
        Self {
            page_no: arg.page_no.clone(),
            page_size: arg.page_size.clone(),
            account: arg.account.clone(),
            name: arg.name.clone(),
        }
    }
}

/// 用户角色添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleAddDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

/// 用户角色修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleEditDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

/// 用户角色分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,

    //默认添加role
    pub resp_set_role: Option<bool>,
}
