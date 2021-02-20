use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use crate::config::CONFIG;

use crate::domain::domain::{LoginCheck, SysRes, SysUser};
use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserPageDTO};
use crate::domain::vo::user::SysUserVO;
use crate::domain::vo::{JWTToken, SignInVO};
use crate::service::CONTEXT;
use crate::util::password_encoder::PasswordEncoder;
use std::collections::HashMap;

///后台用户服务
pub struct SysUserService {}

impl SysUserService {
    /// 后台用户分页
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<SysUser>> {
        let wrapper = CONTEXT
            .rbatis
            .new_wrapper()
            .do_if(arg.name.is_some(), |w| w.eq("name", &arg.name))
            .do_if(arg.account.is_some(), |w| w.eq("account", &arg.account));
        let mut result: Page<SysUser> = CONTEXT
            .rbatis
            .fetch_page_by_wrapper(
                "",
                &wrapper,
                &PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10)),
            )
            .await?;
        for x in &mut result.records {
            x.password = None; //屏蔽密码
        }
        return Ok(result);
    }

    ///用户详情
    pub async fn detail(&self, arg: &IdDTO) -> Result<SysUserVO> {
        let user_id = arg.id.clone().unwrap_or_default();
        let user = self
            .find(&user_id)
            .await?
            .ok_or_else(|| Error::from(format!("用户:{:?} 不存在！", user_id)))?;
        let mut user_vo = SysUserVO::from(user);
        let all_res = CONTEXT.sys_res_service.finds_all_map().await?;
        let roles = CONTEXT
            .sys_user_role_service
            .find_user_roles(&user_id, &all_res)
            .await?;
        user_vo.roles = roles;
        return Ok(user_vo);
    }

    ///后台用户根据id查找
    pub async fn find(&self, id: &str) -> Result<Option<SysUser>> {
        let wrapper = CONTEXT.rbatis.new_wrapper().eq("id", id);
        return CONTEXT.rbatis.fetch_by_wrapper("", &wrapper).await;
    }

    ///根据账户名查找
    pub async fn find_by_account(&self, account: &str) -> Result<Option<SysUser>> {
        let wrapper = CONTEXT.rbatis.new_wrapper().eq("account", account);
        return CONTEXT.rbatis.fetch_by_wrapper("", &wrapper).await;
    }

    ///添加后台账号
    pub async fn add(&self, arg: &UserAddDTO) -> Result<u64> {
        if arg.account.is_none()
            || arg.password.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.password.as_ref().unwrap().is_empty()
        {
            return Err(Error::from("用户名密码不能为空!"));
        }
        let old_user = self
            .find_by_account(arg.account.as_ref().unwrap_or(&"".to_string()))
            .await?;
        if old_user.is_some() {
            return Err(Error::from(format!(
                "用户账户:{}已存在!",
                arg.account.as_ref().unwrap()
            )));
        }
        let id = rbatis::plugin::snowflake::async_snowflake_id()
            .await
            .to_string();
        let user = SysUser {
            id: Some(id.to_string()),
            account: arg.account.clone(),
            password: Some(PasswordEncoder::encode(arg.password.as_ref().unwrap())),
            name: arg.name.clone(),
            login_check: arg.login_check.clone(),
            del: Some(0),
            create_date: Some(NaiveDateTime::now()),
        };
        return Ok(CONTEXT.rbatis.save("", &user).await?.rows_affected);
    }

    ///登陆后台
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        let user: Option<SysUser> = CONTEXT
            .rbatis
            .fetch_by_wrapper(
                "",
                &CONTEXT.rbatis.new_wrapper().eq("account", &arg.account),
            )
            .await?;
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", arg.account)))?;
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
                    return Err(Error::from("密码不正确!"));
                }
            }
            LoginCheck::PasswordQRCodeCheck | LoginCheck::PasswordImgCodeCheck => {
                //check img code
                let cache_code = CONTEXT
                    .redis_service
                    .get_string(&format!("captch:account_{}", &arg.account))
                    .await?;
                if cache_code.eq(&arg.vcode) {
                    return Err(Error::from("验证码不正确!"));
                }
                // check pwd
                if !PasswordEncoder::verify(
                    user.password
                        .as_ref()
                        .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
                    &arg.password,
                ) {
                    return Err(Error::from("密码不正确!"));
                }
            }
            LoginCheck::PhoneCodeCheck => {
                //短信验证码登录
                let sms_code = CONTEXT
                    .redis_service
                    .get_string(&format!(
                        "{}{}",
                        CONFIG.sms_redis_send_key_prefix, &arg.account
                    ))
                    .await?;
                if sms_code.eq(&arg.vcode) {
                    return Err(Error::from("验证码不正确!"));
                }
                //TODO 是否需要删除redis的短信缓存？
            }
        }
        return self.get_user_info(&user).await;
    }

    pub async fn get_user_info_by_token(&self, token: &JWTToken) -> Result<SignInVO> {
        let user: Option<SysUser> = CONTEXT
            .rbatis
            .fetch_by_wrapper("", &CONTEXT.rbatis.new_wrapper().eq("id", &token.id))
            .await?;
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
            user: Some(user.clone()),
            permissions: vec![],
            access_token: String::new(),
            roles: vec![],
        };
        //提前查找所有权限，避免在各个函数方法中重复查找
        let all_res = CONTEXT.sys_res_service.finds_all_map().await?;
        sign_vo.permissions = self.loop_load_level_permission(&user_id, &all_res).await?;
        let jwt_token = JWTToken {
            id: user.id.clone().unwrap_or(String::new()),
            account: user.account.clone().unwrap_or(String::new()),
            permissions: sign_vo.permissions.clone(),
            role_ids: vec![],
            exp: 10000000000,
        };
        sign_vo.access_token = jwt_token.create_token(&CONFIG.jwt_secret)?;
        sign_vo.roles = CONTEXT
            .sys_user_role_service
            .find_user_roles(
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
        let mut user = SysUser {
            id: arg.id.clone(),
            account: arg.account.clone(),
            password: pwd,
            name: arg.name.clone(),
            login_check: arg.login_check.clone(),
            del: None,
            create_date: None,
        };
        CONTEXT.rbatis.update_by_id("", &mut user).await
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        if id.is_empty() {
            return Err(Error::from("id 不能为空！"));
        }
        CONTEXT
            .rbatis
            .remove_by_id::<SysUser>("", &id.to_string())
            .await
    }

    ///递归查找层级结构权限
    pub async fn loop_load_level_permission(
        &self,
        user_id: &str,
        all_res: &HashMap<String, SysRes>,
    ) -> Result<Vec<String>> {
        return CONTEXT
            .sys_role_service
            .find_user_permission(user_id, all_res)
            .await;
    }
}
