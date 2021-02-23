use crate::domain::domain::SysRole;
use crate::domain::vo::SysResVO;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysRoleVO {
    pub id: Option<String>,
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
    pub resources: Vec<SysResVO>,
    pub childs: Vec<SysRoleVO>,
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
            childs: vec![],
            resource_ids: vec![]
        }
    }
}
