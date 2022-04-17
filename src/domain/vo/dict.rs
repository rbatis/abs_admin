use crate::domain::domain::SysDict;
///权限资源表
#[crud_table(table_name: "sys_dict" | table_columns: "id,name,code,state")]
#[derive(Clone, Debug)]
pub struct SysDictVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<rbatis::DateTimeNative>,
}

impl From<SysDict> for SysDictVO {
    fn from(arg: SysDict) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            code: arg.code,
            state: arg.state,
            create_date: arg.create_date,
        }
    }
}


impl SysDictVO {}
