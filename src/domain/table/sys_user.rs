use crate::domain::dto::UserPageDTO;
use crate::domain::table::LoginCheck;
use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

///Background user table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize,Default)]
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

#[html_sql("src/domain/table/sys_user.html")]
impl SysUser {
    pub async fn select_page(conn: &dyn Executor, page_request: &dyn PageRequest, dto: &UserPageDTO) -> rbatis::Result<Page<SysUser>> { impled!() }
}
