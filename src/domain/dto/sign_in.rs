use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub account: String,
    pub password: String,
    //check code...
    pub vcode: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatpchaDTO {
    pub account: Option<String>,
}
