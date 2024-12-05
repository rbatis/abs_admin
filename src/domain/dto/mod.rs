pub mod auth;
pub mod dict;
pub mod sign_in;
pub mod user;

pub mod rbac;

pub use auth::*;
pub use dict::*;
pub use sign_in::*;
pub use user::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}
