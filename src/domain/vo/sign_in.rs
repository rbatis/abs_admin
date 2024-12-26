use crate::domain::table::{LoginCheck};

use crate::context::CONTEXT;
use serde::{Deserialize, Serialize};
use crate::domain::table::sys_user::SysUser;
use crate::domain::vo::rbac::SysRoleVO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub create_date: Option<String>,
    pub permissions: Vec<String>,
    pub access_token: String,
    pub roles: Vec<SysRoleVO>,
}

impl From<SysUser> for SignInVO {
    fn from(value: SysUser) -> Self {
        Self {
            id: value.id,
            account: value.account,
            password: value.password,
            name: value.name,
            login_check: value.login_check,
            state: value.state,
            create_date: value
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            permissions: vec![],
            access_token: "".to_string(),
            roles: vec![],
        }
    }
}
