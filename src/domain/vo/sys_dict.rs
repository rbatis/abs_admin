use crate::context::CONTEXT;
use crate::domain::table::sys_dict::SysDict;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<String>,
}

impl From<SysDict> for SysDictVO {
    fn from(arg: SysDict) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            code: arg.code,
            state: arg.state,
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
        }
    }
}

impl SysDictVO {}
