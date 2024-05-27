use crate::domain::table::SysRole;
use crate::domain::vo::SysPermissionVO;
use crate::service::CONTEXT;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRoleVO {
    pub id: Option<String>,
    pub name: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub create_date: Option<String>,
    pub resources: Vec<SysPermissionVO>,
    pub childs: Option<Vec<SysRoleVO>>,
    pub resource_ids: Vec<String>,
}

impl From<SysRole> for SysRoleVO {
    fn from(arg: SysRole) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            resources: vec![],
            childs: None,
            resource_ids: vec![],
        }
    }
}

impl SysRoleVO {
    pub fn from_option(arg: Option<SysRole>) -> Option<SysRoleVO> {
        arg.map(SysRoleVO::from)
    }
}
