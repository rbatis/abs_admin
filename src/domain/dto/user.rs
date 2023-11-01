use crate::domain::table::{LoginCheck, SysUser, SysUserRole};
use crate::util::password_encoder::PasswordEncoder;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::sql::PageRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub role_id: Option<String>,
    pub state: Option<i32>,
}

impl From<UserAddDTO> for SysUser {
    fn from(arg: UserAddDTO) -> Self {
        SysUser {
            id: ObjectId::new().to_string().into(),
            account: arg.account.clone(),
            password: PasswordEncoder::encode(&arg.password.unwrap_or_default()).into(),
            name: arg.name.clone(),
            login_check: arg.login_check.clone(),
            state: Some(arg.state.unwrap_or(1)),
            create_date: DateTime::now().into(),
        }
    }
}

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

impl From<UserEditDTO> for SysUser {
    fn from(arg: UserEditDTO) -> Self {
        SysUser {
            id: arg.id,
            account: arg.account,
            password: arg.password,
            name: arg.name,
            login_check: arg.login_check,
            state: arg.state,
            create_date: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,
}

impl From<&UserPageDTO> for PageRequest {
    fn from(arg: &UserPageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleAddDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

impl From<UserRoleAddDTO> for SysUserRole {
    fn from(arg: UserRoleAddDTO) -> Self {
        SysUserRole {
            id: arg.id.clone(),
            user_id: arg.user_id.clone(),
            role_id: arg.role_id.clone(),
            create_date: DateTime::now().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,

    //enable set role
    pub resp_set_role: Option<bool>,
}

impl From<&UserRolePageDTO> for PageRequest {
    fn from(arg: &UserRolePageDTO) -> Self {
        PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10))
    }
}
