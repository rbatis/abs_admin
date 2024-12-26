use crate::domain::dto::DictPageDTO;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql_select_page};

///dictionary table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

crud!(SysDict {});
impl SysDict {
    htmlsql_select_page!(select_page(dto:&DictPageDTO) -> SysDict => "src/domain/table/sys_dict.html");
}
