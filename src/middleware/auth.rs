use crate::domain::vo::JWTToken;
use crate::error::Error;
use crate::service::CONTEXT;
pub struct Auth;

///Whether the interface is in the whitelist
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x.contains(path) {
            return true;
        }
    }
    false
}

///Check whether the token is valid and has not expired
pub fn checked_token(token: &str) -> Result<JWTToken, Error> {
    //check token alive
    let token = JWTToken::verify(token);
    match token {
        Ok(token) => {
            Ok(token)
        }
        Err(e) => {
            Err(Error::from(e.to_string()))
        }
    }
}

///Permission to check
pub async fn check_auth(token: &JWTToken, path: &str) -> Result<(), Error> {
    let sys_permission = CONTEXT.sys_permission_service.finds_all_cache().await?;
    for token_permission in &token.permissions {
        for x in &sys_permission {
            if let Some(permission) = &x.permission {
                if let Some(x_path) = &x.path {
                    if permission.eq(token_permission) && path.contains(x_path) {
                        return Ok(());
                    }
                }
            }
        }
    }
    Err(crate::error_info!("access_denied"))
}
