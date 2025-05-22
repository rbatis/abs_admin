use crate::context::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use rbatis::page::{Page, PageRequest};
use rbatis::rbdc::DateTime;

use crate::domain::dto::rbac::UserRoleAddDTO;
use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserPageDTO, UserRolePageDTO};
use crate::domain::table::{LoginCheck};
use crate::domain::vo::sys_user::SysUserVO;
use crate::domain::vo::{JWTToken, SignInVO};
use crate::service::SetUserVO;
use crate::util::password_encoder::PasswordEncoder;
use crate::{error_info, pool};
use std::time::Duration;
use rbs::value;
use crate::domain::table::sys_user::SysUser;

const CACHE_KEY_RETRY: &'static str = "login:login_retry";

///Background User Service
pub struct SysUserService {}

impl SysUserService {
    pub async fn role_page(&self, dto: &UserRolePageDTO) -> Result<Page<SysUserVO>> {
        let mut vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(dto))
            .await?;
        let mut roles = Vec::with_capacity(vo.records.len());
        for x in &vo.records {
            roles.push(SetUserVO {
                id: x.id.clone(),
                roles: vec![],
            });
        }
        CONTEXT.rbac_user_role_service.set_roles(&mut roles).await?;
        let mut idx = 0;
        for x in roles {
            vo.records[idx].roles = x.roles;
            idx += 1;
        }
        Ok(vo)
    }
}

impl SysUserService {
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page: Page<SysUser> = SysUser::select_page(
            pool!(),
            &PageRequest::from(arg),
            arg.name.as_deref().unwrap_or_default(),
            arg.account.as_deref().unwrap_or_default(),
        )
        .await?;
        let page = Page::<SysUserVO>::from(sys_user_page);
        Ok(page)
    }

    ///user details
    pub async fn detail(&self, arg: &IdDTO) -> Result<SysUserVO> {
        let user_id = arg.id.as_deref().unwrap_or_default();
        let user = self.find(&user_id).await?.ok_or_else(|| {
            Error::from(format!("{}={}", error_info!("user_not_exists"), user_id))
        })?;
        let mut user_vo = SysUserVO::from(user);
        let roles = CONTEXT
            .rbac_user_role_service
            .find_user_role(&user_id)
            .await?;
        user_vo.roles = roles;
        Ok(user_vo)
    }

    pub async fn find(&self, id: &str) -> Result<Option<SysUser>> {
        Ok(SysUser::select_by_map(pool!(), value! {"id":id})
            .await?
            .into_iter()
            .next())
    }

    pub async fn find_by_account(&self, account: &str) -> Result<Option<SysUser>> {
        Ok(SysUser::select_by_map(pool!(), value! {"account": account})
            .await?
            .into_iter()
            .next())
    }

    pub async fn add(&self, mut arg: UserAddDTO) -> Result<u64> {
        if arg.account.is_none()
            || arg.account.as_deref().unwrap_or_default().is_empty()
            || arg.name.is_none()
            || arg.name.as_deref().unwrap_or_default().is_empty()
        {
            return Err(Error::from(error_info!("user_and_name_cannot_empty")));
        }
        let old_user = self
            .find_by_account(arg.account.as_deref().unwrap_or_default())
            .await?;
        if old_user.is_some() {
            return Err(Error::from(format!(
                "用户账户:{}已存在!",
                arg.account.as_deref().unwrap_or_default()
            )));
        }
        let mut password = arg.password.as_deref().unwrap_or_default().to_string();
        if password.is_empty() {
            //default password
            password = "123456".to_string();
        }
        arg.password = Some(password);
        let role_id = arg.role_id.clone();
        let user = SysUser::from(arg);
        if role_id.is_some() {
            CONTEXT
                .rbac_user_role_service
                .add(UserRoleAddDTO {
                    id: None,
                    user_id: user.id.clone(),
                    role_id: role_id,
                })
                .await?;
        }
        Ok(SysUser::insert(pool!(), &user).await?.rows_affected)
    }

    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        self.is_need_wait_login_ex(&arg.account).await?;
        let user: Option<SysUser> = SysUser::select_by_map(pool!(), value! {"account": &arg.account})
            .await?
            .into_iter()
            .next();
        let user = user.ok_or_else(|| {
            Error::from(format!(
                "{}={}",
                error_info!("account_not_exists"),
                arg.account
            ))
        })?;
        if user.state.eq(&Some(0)) {
            return Err(Error::from(error_info!("account_disabled")));
        }
        let mut error = None;
        match user
            .login_check
            .as_ref()
            .unwrap_or(&LoginCheck::PasswordCheck)
        {
            LoginCheck::NoCheck => {
                //no check
            }
            LoginCheck::PasswordCheck => {
                // check pwd
                if !PasswordEncoder::verify(
                    user.password
                        .as_ref()
                        .ok_or_else(|| Error::from(error_info!("password_empty")))?,
                    &arg.password,
                ) {
                    error = Some(Error::from(error_info!("password_error")));
                }
            }
            LoginCheck::PasswordImgCodeCheck => {
                //check img code
                let cache_code = CONTEXT
                    .cache_service
                    .get_string(&format!("captch:account_{}", &arg.account))
                    .await?;
                if arg.vcode == ""
                    || cache_code
                        .to_lowercase()
                        .as_str()
                        .ne(arg.vcode.to_lowercase().as_str())
                {
                    error = Some(Error::from(error_info!("vcode_error")))
                }
                // check pwd
                if error.is_none()
                    && !PasswordEncoder::verify(
                        user.password
                            .as_ref()
                            .ok_or_else(|| Error::from(error_info!("password_empty")))?,
                        &arg.password,
                    )
                {
                    error = Some(Error::from(error_info!("password_error")));
                }
            }
            LoginCheck::PhoneCodeCheck => {
                let sms_code = CONTEXT
                    .cache_service
                    .get_string(&format!(
                        "{}{}",
                        CONTEXT.config.sms_cache_send_key_prefix, &arg.account
                    ))
                    .await?;
                if !sms_code.eq(&arg.vcode) {
                    error = Some(Error::from(error_info!("vcode_error")));
                }
            }
        }
        if let Some(e) = &error {
            self.add_retry_login_limit_num(&arg.account).await?;
            return Err(e.clone());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        Ok(sign_in_vo)
    }

    ///is need to wait
    pub async fn is_need_wait_login_ex(&self, account: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT
                .cache_service
                .get_json(&format!("{}{}", CACHE_KEY_RETRY, account))
                .await?;
            if num.unwrap_or(0) >= CONTEXT.config.login_fail_retry {
                let wait_sec: i64 = CONTEXT
                    .cache_service
                    .ttl(&format!("{}{}", CACHE_KEY_RETRY, account))
                    .await?;
                if wait_sec > 0 {
                    let mut e = error_info!("req_frequently");
                    e = e.replace("{}", &format!("{}", wait_sec));
                    return Err(Error::from(e));
                }
            }
        }
        Ok(())
    }

    ///Add redis retry record
    pub async fn add_retry_login_limit_num(&self, account: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT
                .cache_service
                .get_json(&format!("{}{}", CACHE_KEY_RETRY, account))
                .await?;
            let mut num = num.unwrap_or(0);
            if num > CONTEXT.config.login_fail_retry {
                num = CONTEXT.config.login_fail_retry;
            }
            num += 1;
            CONTEXT
                .cache_service
                .set_string_ex(
                    &format!("{}{}", CACHE_KEY_RETRY, account),
                    &num.to_string(),
                    Some(Duration::from_secs(
                        CONTEXT.config.login_fail_retry_wait_sec,
                    )),
                )
                .await?;
        }
        Ok(())
    }

    pub async fn get_user_info_by_token(&self, token: &JWTToken) -> Result<SignInVO> {
        let user = SysUser::select_by_map(pool!(), value! {"id": &token.id})
            .await?
            .into_iter()
            .next();
        let user = user.ok_or_else(|| {
            Error::from(format!(
                "{}:{}",
                error_info!("account_not_exists"),
                token.account
            ))
        })?;
        self.get_user_info(&user).await
    }

    pub async fn get_user_info(&self, user: &SysUser) -> Result<SignInVO> {
        let mut user = user.clone();
        user.password = None;
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from(error_info!("id_empty")))?;
        let mut sign_vo = SignInVO::from(user);
        sign_vo.permissions = self.load_level_permission(&user_id).await?;
        let jwt_token = JWTToken {
            id: sign_vo.id.clone().unwrap_or_default(),
            account: sign_vo.account.clone().unwrap_or_default(),
            permissions: sign_vo.permissions.clone(),
            role_ids: vec![],
            exp: DateTime::now().unix_timestamp() as usize + CONTEXT.config.jwt_exp,
        };
        sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
        sign_vo.roles = CONTEXT
            .rbac_user_role_service
            .find_user_role(&sign_vo.id.clone().unwrap_or_default())
            .await?;
        Ok(sign_vo)
    }

    pub async fn sign_out(&self) {}

    pub async fn edit(&self, arg: UserEditDTO) -> Result<u64> {
        let role_id = arg.role_id.clone();
        let mut arg = SysUser::from(arg);
        //old user
        let user = SysUser::select_by_map(pool!(), value! {"id": arg.id.as_ref()})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(error_info!("user_cannot_find")))?;
        //do not update account
        arg.account = None;
        let mut password = None;
        if arg.password != user.password {
            if let Some(pass) = arg.password.as_ref() {
                password = Some(PasswordEncoder::encode(pass));
            }
        }
        arg.password = password;
        if role_id.is_some() {
            CONTEXT
                .rbac_user_role_service
                .add(UserRoleAddDTO {
                    id: None,
                    user_id: arg.id.clone(),
                    role_id: role_id,
                })
                .await?;
        }
        Ok(SysUser::update_by_map(pool!(), &arg, value! {"id":&arg.id})
            .await?
            .rows_affected)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        if id.is_empty() {
            return Err(Error::from(error_info!("id_empty")));
        }
        let r = SysUser::delete_by_map(pool!(), value! {"id": id}).await?;
        CONTEXT.rbac_user_role_service.remove_by_user_id(id).await?;
        Ok(r.rows_affected)
    }

    ///Find user-authority hierarchy permissions
    pub async fn load_level_permission(&self, user_id: &str) -> Result<Vec<String>> {
        let data = CONTEXT
            .rbac_user_role_service
            .find_user_role(user_id)
            .await?;
        let mut perms= Vec::with_capacity(data.len());
        for x in data {
            for x in x.permissions {
                perms.push(x.permission.clone().unwrap_or_default());
            }
        }
        Ok(perms)
    }
}
