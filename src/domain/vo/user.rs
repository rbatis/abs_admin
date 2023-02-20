use serde::{Deserialize, Serialize};
use crate::domain::table::SysUser;
use crate::domain::vo::SysRoleVO;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserVO {
    #[serde(flatten)]
    pub inner: SysUser,
    pub role: Option<SysRoleVO>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            inner:arg,
            role: None,
        }
    }
}
