#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum LoginCheck {
    NoCheck,
    Password,
    PasswordImgCode,
    PasswordImgCodeFirstTime,
    PhoneCode,
    PhoneCodeImgCode,
    PhoneCodeImgCodeFirstTime,
}