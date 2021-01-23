use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use crate::dao::RB;
use crate::domain::domain::SysUser;
use crate::domain::dto::{SignInDTO, UserAddDTO, UserEditDTO, UserPageDTO};
use crate::domain::vo::SignInVO;
use crate::service::REDIS_SERVICE;
use crate::service::SYS_ROLE_SERVICE;
use crate::util::password_encoder::PasswordEncoder;

///后台用户服务
pub struct SysUserService {}

impl SysUserService {
    /// 后台用户分页
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<SysUser>> {
        let wrapper = RB
            .new_wrapper()
            .do_if(arg.name.is_some(), |w| w.eq("name", &arg.name))
            .do_if(arg.account.is_some(), |w| w.eq("account", &arg.account))
            .check()?;
        let mut result: Page<SysUser> = RB
            .fetch_page_by_wrapper(
                "",
                &wrapper,
                &PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10)),
            )
            .await?;
        for x in &mut result.records {
            x.password = None; //屏蔽密码
        }
        return Ok(result);
    }

    ///后台用户根据id查找
    pub async fn find(&self, id: &str) -> Result<Option<SysUser>> {
        let wrapper = RB.new_wrapper().eq("id", id).check()?;
        return RB.fetch_by_wrapper("", &wrapper).await;
    }

    ///根据账户名查找
    pub async fn find_by_account(&self, account: &str) -> Result<Option<SysUser>> {
        let wrapper = RB.new_wrapper().eq("account", account).check()?;
        return RB.fetch_by_wrapper("", &wrapper).await;
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
        return Ok(RB.save("", &user).await?.rows_affected);
    }

    ///登陆后台
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        //check img code
        let cache_img_code = REDIS_SERVICE
            .get_string(&format!("captch:account_{}", &arg.account))
            .await?;
        if cache_img_code.eq(&arg.img_code) {
            return Err(Error::from("验证码不正确!"));
        }
        let user: Option<SysUser> = RB
            .fetch_by_wrapper("", &RB.new_wrapper().eq("account", &arg.account).check()?)
            .await?;
        let mut user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", arg.account)))?;
        // check pwd
        if !PasswordEncoder::verify(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            &arg.password,
        ) {
            return Err(Error::from("密码不正确!"));
        }
        user.password = None; //去除密码，增加安全性
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let mut sign_vo = SignInVO {
            user: Some(user),
            permissions: vec![],
        };
        sign_vo.permissions = self.loop_load_permission(&user_id).await?;
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
            account: arg.account.clone(),
            password: pwd,
            name: arg.name.clone(),
            login_check: arg.login_check.clone(),
            del: None,
            create_date: None,
        };
        RB.update_by_id("", &user).await
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        if id.is_empty() {
            return Err(Error::from("id 不能为空！"));
        }
        RB.remove_by_id::<SysUser>("", &id.to_string()).await
    }

    ///循环查找权限
    pub async fn loop_load_permission(&self, user_id: &str) -> Result<Vec<String>> {
        return SYS_ROLE_SERVICE.find_user_permission(user_id).await;
    }
}
