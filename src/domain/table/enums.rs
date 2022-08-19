use serde::{Deserializer, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum LoginCheck {
    NoCheck,
    PasswordCheck,
    PasswordImgCodeCheck,
    PhoneCodeCheck,
}

impl Display for LoginCheck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginCheck::NoCheck => f.write_str(""),
            LoginCheck::PasswordCheck => f.write_str("PasswordCheck"),
            LoginCheck::PasswordImgCodeCheck => f.write_str("PasswordImgCodeCheck"),
            LoginCheck::PhoneCodeCheck => f.write_str("PhoneCodeCheck"),
        }
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
        Ok(match v.as_str() {
            "" => LoginCheck::NoCheck,
            "NoCheck" => LoginCheck::NoCheck,
            "PasswordCheck" => LoginCheck::PasswordCheck,
            "PasswordImgCodeCheck" => LoginCheck::PasswordImgCodeCheck,
            "PhoneCodeCheck" => LoginCheck::PhoneCodeCheck,
            _ => LoginCheck::NoCheck,
        })
    }
}
