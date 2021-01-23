#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum LoginCheck {
    NoCheck,
    PASSWORD,
    PasswordImgcode(i32),
    PhoneCode,
    PhoneCodeImgcode(i32),
}

#[cfg(test)]
mod test {
    use crate::domain::domain::LoginCheck;

    #[test]
    fn test_serialize() {
        let t = LoginCheck::PasswordImgcode(1);
        let js = serde_json::json!(t);
        println!("{}", js.to_string());
    }

    #[test]
    fn test_serialize2() {
        let t = LoginCheck::NoCheck;
        let js = serde_json::json!(t);
        println!("{}", js.to_string());
    }
}