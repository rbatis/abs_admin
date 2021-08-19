use serde::{Deserialize, Serialize};

/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub account: String,
    pub password: String,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    pub vcode: String,
}

/// 验证码
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatpchaDTO {
    pub account: Option<String>,
}
