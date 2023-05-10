use crate::domain::table::SysRole;
use crate::domain::vo::SysPermissionVO;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRoleVO {
    #[serde(flatten)]
    pub inner: SysRole,
    pub resources: Vec<SysPermissionVO>,
    pub childs: Option<Vec<SysRoleVO>>,
    pub resource_ids: Vec<String>,
}

impl From<SysRole> for SysRoleVO {
    fn from(arg: SysRole) -> Self {
        Self {
            inner: arg,
            resources: vec![],
            childs: None,
            resource_ids: vec![],
        }
    }
}

impl SysRoleVO {
    pub fn from_option(arg: Option<SysRole>) -> Option<SysRoleVO> {
        match arg {
            Some(arg) => Some(SysRoleVO {
                inner: arg,
                resources: vec![],
                childs: None,
                resource_ids: vec![],
            }),
            _ => None,
        }
    }
}
