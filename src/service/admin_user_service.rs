use chrono::Utc;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::wrapper::Wrapper;
use rbatis_core::Error;
use rbatis_core::Result;
use uuid::Uuid;

use crate::dao::RB;
use crate::domain::domain::BizAdminUser;
use crate::domain::dto::{SignInDTO, UserAddDTO, UserPageDTO};
use crate::domain::vo::SignInVO;
use crate::util::password_encoder::PasswordEncoder;

///后台用户服务
pub struct AdminUserService {}

impl AdminUserService {
    /// 后台用户分页
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<BizAdminUser>> {
        let mut w = Wrapper::new(&RB.driver_type()?);
        if arg.name.is_some() {
            w.eq("name", &arg.name.clone().unwrap());
        }
        if arg.account.is_some() {
            w.eq("account", &arg.account.clone().unwrap());
        }
        w = w.check()?;
        return Ok(RB.fetch_page_by_wrapper("", &w, &PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10))).await?);
    }

    ///后台用户根据id查找
    pub async fn find(&self, id: &str) -> Result<Option<BizAdminUser>> {
        let mut w = Wrapper::new(&RB.driver_type()?)
            .eq("id",id)
            .check()?;
       return RB.fetch_by_wrapper("",&w).await;
    }

    ///根据账户名查找
    pub async fn find_by_account(&self, account: &str) -> Result<Option<BizAdminUser>> {
        let mut w = Wrapper::new(&RB.driver_type()?)
            .eq("account",account)
            .check()?;
        return RB.fetch_by_wrapper("",&w).await;
    }


    ///添加后台账号
    pub async fn add(&self, arg: &UserAddDTO) -> Result<u64> {
        if arg.account.is_none() || arg.password.is_none() || arg.account.as_ref().unwrap().is_empty() || arg.password.as_ref().unwrap().is_empty() {
            return Err(Error::from("用户名密码不能为空!"));
        }
        let old_user = self.find_by_account(arg.account.as_ref().unwrap_or(&"".to_string())).await?;
        if old_user.is_some(){
            return Err(Error::from(format!("用户账户:{}已存在!",arg.account.as_ref().unwrap())));
        }
        let id = Uuid::new_v4();
        let dt = Utc::now();
        let user = BizAdminUser {
            id: Some(id.to_string()),
            account: arg.account.clone(),
            password: Some(PasswordEncoder::encode(arg.password.as_ref().unwrap())),
            name: arg.name.clone(),
            del: Some(1),
            create_time: Some(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        };
        return RB.save("", &user).await;
    }

    ///登陆后台
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        if arg.account.is_none() || arg.password.is_none() || arg.account.as_ref().unwrap().is_empty() || arg.password.as_ref().unwrap().is_empty() {
            return Err(Error::from("用户名密码不能为空!"));
        }
        let w = Wrapper::new(&RB.driver_type()?)
            .eq("account", &arg.account)
            .check()?;
        let user: Option<BizAdminUser> = RB.fetch_by_wrapper("", &w).await?;
        if user.is_none() {
            return Err(Error::from(format!("账号:{} 不存在!", arg.account.as_ref().unwrap())));
        }
        let mut user = user.unwrap();
        // check pwd
        if !PasswordEncoder::verify(user.password.as_ref().unwrap(), arg.password.as_ref().unwrap()) {
            return Err(Error::from("密码不正确!"));
        }
        user.password = None;//去除密码，增加安全性
        let sign_vo = SignInVO {
            user: Some(user),
            permissions: vec![],
        };
        //TODO load permission
        self.loop_load_permission("").await;
        return Ok(sign_vo);
    }

    ///登出后台
    pub async fn sign_out(&self) {}

    ///循环查找权限
    pub async fn loop_load_permission(&self, id: &str) {}
}
