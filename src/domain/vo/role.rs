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
}
