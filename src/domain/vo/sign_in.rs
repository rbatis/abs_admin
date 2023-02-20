use crate::domain::vo::SysRoleVO;
use serde::{Deserialize, Serialize};
use crate::domain::table::SysUser;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    #[serde(flatten)]
    pub inner: SysUser,
    pub permissions: Vec<String>,
    pub access_token: String,
    pub role: Option<SysRoleVO>,
}
