#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum LoginCheck {
    NoCheck,
    PASSWORD,
    PasswordImgCode,
    PasswordImgCodeFirstTime,
    PhoneCode,
    PhoneCodeImgCode,
    PhoneCodeImgCodeFirstTime,
}