use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::JWTToken;
use crate::error::Result;

pub struct SysAuthService {}

impl SysAuthService {
    pub async fn check_auth(&self, arg: SysAuthDTO) -> Result<JWTToken> {
        let jwt = crate::middleware::auth::checked_token(&arg.access_token, &arg.path).await?;
        crate::middleware::auth::check_auth(&jwt, &arg.path).await?;
        return Ok(jwt);
    }
}
