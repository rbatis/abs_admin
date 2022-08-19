use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use rbatis::rbdc::types::datetime::FastDateTime;
use rbatis::sql::page::{Page, PageRequest};

use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserPageDTO, UserRoleAddDTO};
use crate::domain::table::{LoginCheck, SysUser};
use crate::domain::vo::user::SysUserVO;
use crate::domain::vo::{JWTToken, SignInVO, SysResVO};
use crate::pool;
use crate::util::password_encoder::PasswordEncoder;
use rbatis::plugin::object_id::ObjectId;
use std::collections::BTreeMap;
use std::time::Duration;

use crate::util::options::OptionStringRefUnwrapOrDefault;

const REDIS_KEY_RETRY: &'static str = "login:login_retry";

///后台用户服务
pub struct SysUserService {}

impl SysUserService {
    /// 后台用户分页
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<SysUserVO>> {
        let sys_user_page: Page<SysUser> = SysUser::select_page(
            pool!(),
            &PageRequest::from(arg),
            arg.name.as_deref().unwrap_or_default(),
            arg.account.as_deref().unwrap_or_default(),
        )
        .await?;
        let page = Page::<SysUserVO>::from(sys_user_page);
        return Ok(page);
    }

    ///用户详情
    pub async fn detail(&self, arg: &IdDTO) -> Result<SysUserVO> {
        let user_id = arg.id.as_deref().unwrap_or_default();
        let user = self
            .find(&user_id)
            .await?
            .ok_or_else(|| Error::from(format!("用户:{:?} 不存在！", user_id)))?;
        let mut user_vo = SysUserVO::from(user);
        let all_res = CONTEXT.sys_res_service.finds_all_map().await?;
        let role = CONTEXT
            .sys_user_role_service
            .find_user_role(&user_id, &all_res)
            .await?;
        user_vo.role = role;
        return Ok(user_vo);
    }

    ///后台用户根据id查找
    pub async fn find(&self, id: &str) -> Result<Option<SysUser>> {
        Ok(
            SysUser::select_by_column(pool!(), field_name!(SysUser.id), id)
                .await?
                .into_iter()
                .next(),
        )
    }

    ///根据账户名查找
    pub async fn find_by_account(&self, account: &str) -> Result<Option<SysUser>> {
        Ok(
            SysUser::select_by_column(pool!(), field_name!(SysUser.account), account)
                .await?
                .into_iter()
                .next(),
        )
    }

    ///添加后台账号
    pub async fn add(&self, arg: &UserAddDTO) -> Result<u64> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.name.is_none()
            || arg.name.as_ref().unwrap().is_empty()
        {
            return Err(Error::from("用户名和姓名不能为空!"));
        }
        let old_user = self
            .find_by_account(arg.account.as_ref().unwrap_or_def())
            .await?;
        if old_user.is_some() {
            return Err(Error::from(format!(
                "用户账户:{}已存在!",
                arg.account.as_ref().unwrap()
            )));
        }
        let mut password = arg.password.as_deref().unwrap_or_default().to_string();
        if password.is_empty() {
            //默认密码
            password = "123456".to_string();
        }
        let id = ObjectId::new().to_string();
        let user = SysUser {
            id: id.to_string().into(),
            account: arg.account.clone(),
            password: PasswordEncoder::encode(&password).into(),
            name: arg.name.clone(),
            login_check: arg.login_check.clone(),
            state: 0.into(),
            del: 0.into(),
            create_date: FastDateTime::now().set_micro(0).into(),
        };
        if let Some(_) = &arg.role_id {
            CONTEXT
                .sys_user_role_service
                .add(&UserRoleAddDTO {
                    id: None,
                    user_id: user.id.clone(),
                    role_id: arg.role_id.clone(),
                })
                .await?;
        }
        Ok(SysUser::insert(pool!(), &user).await?.rows_affected)
    }

    ///登陆后台
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        self.is_need_wait_login_ex().await?;
        let user: Option<SysUser> =
            SysUser::select_by_column(pool!(), field_name!(SysUser.account), &arg.account)
                .await?
                .into_iter()
                .next();
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", arg.account)))?;
        if user.state.eq(&Some(0)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        match user
            .login_check
            .as_ref()
            .unwrap_or(&LoginCheck::PasswordCheck)
        {
            LoginCheck::NoCheck => {
                //无校验登录，适合Debug用
            }
            LoginCheck::PasswordCheck => {
                // check pwd
                if !PasswordEncoder::verify(
                    user.password
                        .as_ref()
                        .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
                    &arg.password,
                ) {
                    error = Some(Error::from("密码不正确!"));
                }
            }
            LoginCheck::PasswordImgCodeCheck => {
                //check img code
                let cache_code = CONTEXT
                    .cache_service
                    .get_string(&format!("captch:account_{}", &arg.account))
                    .await?;
                if cache_code.eq(&arg.vcode) {
                    error = Some(Error::from("验证码不正确!"))
                }
                // check pwd
                if !PasswordEncoder::verify(
                    user.password
                        .as_ref()
                        .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
                    &arg.password,
                ) {
                    error = Some(Error::from("密码不正确!"));
                }
            }
            LoginCheck::PhoneCodeCheck => {
                //短信验证码登录
                let sms_code = CONTEXT
                    .cache_service
                    .get_string(&format!(
                        "{}{}",
                        CONTEXT.config.sms_cache_send_key_prefix, &arg.account
                    ))
                    .await?;
                if sms_code.eq(&arg.vcode) {
                    error = Some(Error::from("验证码不正确!"));
                }
            }
        }
        if error.is_some() {
            self.add_retry_login_limit_num().await?;
            return Err(error.unwrap());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        return Ok(sign_in_vo);
    }

    ///是否需要等待
    pub async fn is_need_wait_login_ex(&self) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(REDIS_KEY_RETRY).await?;
            if num.unwrap_or(0) >= CONTEXT.config.login_fail_retry {
                let wait_sec: i64 = CONTEXT.cache_service.ttl(REDIS_KEY_RETRY).await?;
                if wait_sec > 0 {
                    return Err(Error::from(format!(
                        "操作过于频繁，请等待{}秒后重试!",
                        wait_sec
                    )));
                }
            }
        }
        return Ok(());
    }

    ///增加redis重试记录
    pub async fn add_retry_login_limit_num(&self) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(REDIS_KEY_RETRY).await?;
            let mut num = num.unwrap_or(0);
            if num > CONTEXT.config.login_fail_retry {
                num = CONTEXT.config.login_fail_retry;
            }
            num += 1;
            CONTEXT
                .cache_service
                .set_string_ex(
                    REDIS_KEY_RETRY,
                    &num.to_string(),
                    Some(Duration::from_secs(
                        CONTEXT.config.login_fail_retry_wait_sec as u64,
                    )),
                )
                .await?;
        }
        return Ok(());
    }

    pub async fn get_user_info_by_token(&self, token: &JWTToken) -> Result<SignInVO> {
        let user = SysUser::select_by_column(pool!(), field_name!(SysUser.id), &token.id)
            .await?
            .into_iter()
            .next();
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", token.account)))?;
        return self.get_user_info(&user).await;
    }

    pub async fn get_user_info(&self, user: &SysUser) -> Result<SignInVO> {
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let mut sign_vo = SignInVO {
            user: Some(user.clone().into()),
            permissions: vec![],
            access_token: String::new(),
            role: None,
        };
        //提前查找所有权限，避免在各个函数方法中重复查找
        let all_res = CONTEXT.sys_res_service.finds_all_map().await?;
        sign_vo.permissions = self.loop_load_level_permission(&user_id, &all_res).await?;
        let jwt_token = JWTToken {
            id: user.id.as_deref().unwrap_or_default().to_string(),
            account: user.account.unwrap_or_default(),
            permissions: sign_vo.permissions.clone(),
            role_ids: vec![],
            exp: FastDateTime::now().set_micro(0).unix_timestamp_millis() as usize,
        };
        sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
        sign_vo.role = CONTEXT
            .sys_user_role_service
            .find_user_role(
                &user.id.unwrap_or_else(|| {
                    return String::new();
                }),
                &all_res,
            )
            .await?;
        return Ok(sign_vo);
    }

    ///登出后台
    pub async fn sign_out(&self) {}

    pub async fn edit(&self, arg: &UserEditDTO) -> Result<u64> {
        let mut pwd = None;
        //源密码加密后再存储
        if arg.password.is_some() {
            pwd = Some(PasswordEncoder::encode(arg.password.as_ref().unwrap()));
        }
        let user = SysUser {
            id: arg.id.clone(),
            account: None,
            password: pwd,
            name: arg.name.clone(),
            login_check: arg.login_check.clone(),
            state: arg.state.clone(),
            del: None,
            create_date: None,
        };
        if arg.role_id.is_some() {
            CONTEXT
                .sys_user_role_service
                .add(&UserRoleAddDTO {
                    id: None,
                    user_id: user.id.clone(),
                    role_id: arg.role_id.clone(),
                })
                .await?;
        }
        Ok(
            SysUser::update_by_column(pool!(), &user, field_name!(SysUser.id))
                .await?
                .rows_affected,
        )
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        if id.is_empty() {
            return Err(Error::from("id 不能为空！"));
        }
        let trash = SysUser::select_by_column(pool!(), field_name!(SysUser.id), id).await?;
        let r = SysUser::delete_by_column(pool!(), field_name!(SysUser.id), id).await?;
        CONTEXT.sys_trash_service.add("sys_user", &trash).await;
        CONTEXT.sys_user_role_service.remove_by_user_id(id).await?;
        return Ok(r.rows_affected);
    }

    ///递归查找层级结构权限
    pub async fn loop_load_level_permission(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<String>> {
        return CONTEXT
            .sys_role_service
            .find_user_permission(user_id, all_res)
            .await;
    }
}
