use crate::domain::domain::SysUser;
use serde::{Deserialize, Serialize};
use crate::domain::vo::{SysRoleVO};

///登录数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub user: Option<SysUser>,
    pub permissions: Vec<String>,
    pub access_token: String,
    pub roles: Vec<SysRoleVO>,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}