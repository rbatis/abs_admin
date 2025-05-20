use crate::domain::vo::JWTToken;
use crate::error_info;
use crate::context::CONTEXT;
pub struct Auth;


///Check whether the token is valid and has not expired
pub fn checked_token(token: &str) -> Result<JWTToken, crate::error::Error> {
    //check token alive
    let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match token {
        Ok(token) => {
            Ok(token)
        }
        Err(e) => {
            Err(crate::error::Error::from(e.to_string()))
        }
    }
}

///Permission to check
pub async fn check_auth(token: &JWTToken, path: &str) -> Result<(), crate::error::Error> {
    let sys_permission = CONTEXT.rbac_permission_service.finds_all().await?;
    for token_permission in &token.permissions {
        for x in &sys_permission {
            match &x.permission {
                Some(permission) => match &x.path {
                    None => {}
                    Some(x_path) => {
                        if permission.eq(token_permission) && path.contains(x_path) {
                            return Ok(());
                        }
                    }
                },
                _ => {}
            }
        }
    }
    Err(crate::error::Error::from(error_info!("access_denied")))
}
