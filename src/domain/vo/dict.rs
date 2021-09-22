use crate::domain::domain::SysDict;
use chrono::NaiveDateTime;
use std::collections::HashMap;

///权限资源表
#[crud_table(table_name: "sys_dict" | table_columns: "id,name,code,state")]
#[derive(Clone, Debug)]
pub struct SysDictVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
}

impl From<&SysDict> for SysDictVO {
    fn from(arg: &SysDict) -> Self {
        Self {
            id: arg.id.clone(),
            name: arg.name.clone(),
            code: arg.code.clone(),
            state: arg.state.clone(),
            create_date: arg.create_date.clone(),
        }
    }
}

impl SysDictVO {}
