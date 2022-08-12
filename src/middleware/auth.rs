use crate::domain::vo::JWTToken;
use crate::service::CONTEXT;
pub struct Auth;

///是否处在白名单接口中
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x.contains(path) {
            return true;
        }
    }
    return false;
}

///校验token是否有效，未过期
pub async fn checked_token(token: &str, path: &str) -> Result<JWTToken, crate::error::Error> {
    //check token alive
    let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match token {
        Ok(token) => {
            return Ok(token);
        }
        Err(e) => {
            return Err(crate::error::Error::from(e.to_string()));
        }
    }
}

///权限校验
pub async fn check_auth(token: &JWTToken, path: &str) -> Result<(), crate::error::Error> {
    let sys_res = CONTEXT.sys_res_service.finds_all().await?;
    //权限校验
    for token_permission in &token.permissions {
        for x in &sys_res {
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
    return Err(crate::error::Error::from("无权限访问!"));
}
