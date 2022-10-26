use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum LoginCheck {
    NoCheck,
    PasswordCheck,
    PasswordImgCodeCheck,
    PhoneCodeCheck,
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
