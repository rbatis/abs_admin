#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum LoginCheck {
    NoCheck,
    PasswordCheck,
    PasswordImgCodeCheck,
    PhoneCodeCheck,
}
