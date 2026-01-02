use crate::domain::table::LoginCheck;
use rbatis::rbdc::DateTime;
use rbatis::{crud, htmlsql_select_page};
use crate::domain::dto::UserPageDTO;

///Background user table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    /// Deletion flag, 0: normal, 1: deleted
    pub deleted: Option<i32>,
    pub create_date: Option<DateTime>,
}

crud!(SysUser {});

impl SysUser {
    htmlsql_select_page!(select_page(dto:&UserPageDTO) -> SysUser => "src/domain/table/sys_user.html");
}
