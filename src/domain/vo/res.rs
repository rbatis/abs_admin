use chrono::NaiveDateTime;
use crate::domain::domain::SysRes;
///权限资源表
#[crud_enable(table_name: "sys_res" | table_columns: "id,parent_id,name,permission,path,del")]
#[derive(Clone, Debug)]
pub struct SysResVO {
    pub id: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //权限
    pub permission: Option<String>,
    //前端-菜单路径
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
    pub childs: Option<Vec<SysResVO>>,
}

impl From<&SysRes> for SysResVO {
    fn from(arg: &SysRes) -> Self {
        Self {
            id: arg.id.clone(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            del: arg.del.clone(),
            create_date: arg.create_date.clone(),
            childs: None,
        }
    }
}