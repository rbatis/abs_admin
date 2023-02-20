use crate::domain::vo::SysRoleVO;
use serde::{Deserialize, Serialize};
use crate::domain::table::SysUser;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    #[serde(flatten)]
    pub user: SysUser,
    pub permissions: Vec<String>,
    pub access_token: String,
    pub role: Option<SysRoleVO>,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
