use crate::domain::table::SysUser;
use crate::domain::vo::SysRoleVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    #[serde(flatten)]
    pub inner: SysUser,
    pub permissions: Vec<String>,
    pub access_token: String,
    pub role: Option<SysRoleVO>,
}
