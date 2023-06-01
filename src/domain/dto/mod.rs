pub mod auth;
pub mod dict;
pub mod permission;
pub mod role;
pub mod sign_in;
pub mod user;

pub use auth::*;
pub use dict::*;
pub use permission::*;
pub use role::*;
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
