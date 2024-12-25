pub mod auth;
pub mod sys_dict;
pub mod sign_in;
pub mod sys_user;

pub mod rbac;

pub use auth::*;
pub use sys_dict::*;
pub use sign_in::*;
pub use sys_user::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}
