use crate::domain::table::{LoginCheck, SysUser};
use crate::domain::vo::SysRoleVO;
use serde::{Deserialize, Serialize};

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
    pub role: Option<SysRoleVO>,
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
            create_date: value.create_date.map(|v| v.display_stand()),
            permissions: vec![],
            access_token: "".to_string(),
            role: None,
        }
    }
}
