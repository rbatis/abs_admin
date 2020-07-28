use crate::domain::dto::{SignInDTO, UserAddDTO, UserPageDTO};
use rbatis_core::Error;
use crate::dao::RB;
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use crate::domain::domain::BizAdminUser;
use crate::domain::vo::SignInVO;
use rbatis_core::Result;
use uuid::Uuid;
use chrono::Utc;
use rbatis::plugin::page::{Page, PageRequest};
use crate::util::password_encoder::PasswordEncoder;

///后台用户服务
pub struct AdminUserService {}

impl AdminUserService {
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<BizAdminUser>> {
        let mut w = Wrapper::new(&RB.driver_type()?);
        if arg.name.is_some() {
            w.eq("name", &arg.name.clone().unwrap());
        }
        if arg.account.is_some() {
            if w.args.len() > 0 {
                w.and();
            }
            w.eq("account", &arg.account.clone().unwrap());
        }
        w = w.check()?;
        return Ok(RB.fetch_page_by_wrapper("", &w, &PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10))).await?);
    }


    ///添加
    pub async fn add(&self, arg: &UserAddDTO) -> Result<u64> {
        if arg.account.is_none() || arg.password.is_none() || arg.account.as_ref().unwrap().is_empty() || arg.password.as_ref().unwrap().is_empty() {
            return Err(Error::from("用户名密码不能为空!"));
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

    ///登陆
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        if arg.account.is_none() || arg.password.is_none() || arg.account.as_ref().unwrap().is_empty() || arg.password.as_ref().unwrap().is_empty() {
            return Err(Error::from("用户名密码不能为空!"));
        }
        let w = Wrapper::new(&RB.driver_type()?).eq("account", &arg.account).check()?;
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

    ///登出
    pub async fn sign_out(&self) {}


    pub async fn loop_load_permission(&self, id: &str) {}
}
