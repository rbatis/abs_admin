use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Sms {
    pub account: String,
    pub args: HashMap<String, String>,
}
