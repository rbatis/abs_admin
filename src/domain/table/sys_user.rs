use rbatis::{crud, impl_select_page};
use crate::domain::table::LoginCheck;
use rbatis::rbdc::DateTime;

///Background user table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

crud!(SysUser {});

impl_select_page!(SysUser{select_page(name:&str,account:&str)=>
    "` where 0 = 0 `
    if name != '':
      ` and name like #{'%'+name+'%'}`
    if account != '':
      ` and account like #{'%'+account+'%'}`
    if do_count == false:
     ` order by create_date desc`"});