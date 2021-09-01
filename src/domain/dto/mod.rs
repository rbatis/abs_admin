pub mod dict;
pub mod res;
pub mod role;
pub mod sign_in;
pub mod user;

pub use dict::*;
pub use res::*;
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
