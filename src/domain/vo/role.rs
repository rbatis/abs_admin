use crate::domain::table::SysRole;
use crate::domain::vo::SysResVO;
use rbatis::rbdc::datetime::FastDateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRoleVO {
    pub id: Option<String>,
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
    pub resources: Vec<SysResVO>,
    pub childs: Option<Vec<SysRoleVO>>,
    pub resource_ids: Vec<String>,
}

impl From<SysRole> for SysRoleVO {
    fn from(arg: SysRole) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            del: arg.del,
            create_date: arg.create_date,
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
                id: arg.id,
                name: arg.name,
                parent_id: arg.parent_id,
                del: arg.del,
                create_date: arg.create_date,
                resources: vec![],
                childs: None,
                resource_ids: vec![],
            }),
            _ => None,
        }
    }
}
