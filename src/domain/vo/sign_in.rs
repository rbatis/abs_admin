use crate::domain::domain::SysUser;
use serde::{Deserialize, Serialize};

///登录数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub user: Option<SysUser>,
    pub permissions: Vec<String>,
    pub access_token: String,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}