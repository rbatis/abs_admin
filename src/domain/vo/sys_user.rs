use crate::domain::table::{LoginCheck};
use crate::domain::vo::rbac::SysRoleVO;
use crate::context::CONTEXT;
use serde::{Deserialize, Serialize};
use crate::domain::table::sys_user::SysUser;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub create_date: Option<String>,
    pub roles: Vec<SysRoleVO>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            id: arg.id,
            account: arg.account,
            password: arg.password,
            name: arg.name,
            login_check: arg.login_check,
            state: arg.state,
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            roles: vec![],
        }
    }
}
