use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sms {
    pub account: String,
    pub args: HashMap<String, String>,
}