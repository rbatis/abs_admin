use crate::domain::dto::UserPageDTO;
use rbatis::{crud, html_sql};
use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use rbatis::rbdc::datetime::DateTime;

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
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &UserPageDTO) -> rbatis::Result<Page<SysUser>> { impled!() }
}


#[derive(Clone)]
pub enum LoginCheck {
    NoCheck,
    PasswordCheck,
    PasswordImgCodeCheck,
    PhoneCodeCheck,
}

impl Default for LoginCheck {
    fn default() -> Self {
        LoginCheck::NoCheck
    }
}

impl From<LoginCheck> for &str {
    fn from(arg: LoginCheck) -> Self {
        match arg {
            LoginCheck::NoCheck => "",
            LoginCheck::PasswordCheck => "PasswordCheck",
            LoginCheck::PasswordImgCodeCheck => "PasswordImgCodeCheck",
            LoginCheck::PhoneCodeCheck => "PhoneCodeCheck",
        }
    }
}

impl From<&str> for LoginCheck {
    fn from(arg: &str) -> Self {
        match arg {
            "" => LoginCheck::NoCheck,
            "NoCheck" => LoginCheck::NoCheck,
            "PasswordCheck" => LoginCheck::PasswordCheck,
            "PasswordImgCodeCheck" => LoginCheck::PasswordImgCodeCheck,
            "PhoneCodeCheck" => LoginCheck::PhoneCodeCheck,
            _ => LoginCheck::NoCheck,
        }
    }
}

impl Debug for LoginCheck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&str>::from(self.clone()))
    }
}

impl Display for LoginCheck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&str>::from(self.clone()))
    }
}

impl serde::Serialize for LoginCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for LoginCheck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = String::deserialize(deserializer)?;
        Ok(LoginCheck::from(v.as_str()))
    }
}
