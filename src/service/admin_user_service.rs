use crate::domain::dto::{SignInDTO, UserAddDTO};
use rbatis_core::Error;
use crate::dao::RB;
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use crate::domain::domain::BizAdminUser;
use crate::domain::vo::SignInVO;
use rbatis_core::Result;
use crate::util::BCryptPasswordEncoder;
use uuid::Uuid;
use chrono::Utc;

///后台用户服务
pub struct AdminUserService {}

impl AdminUserService {
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
            password: Some(BCryptPasswordEncoder::encode(arg.password.as_ref().unwrap())),
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
        let user = user.unwrap();
        // check pwd
        if !BCryptPasswordEncoder::verify(user.password.as_ref().unwrap(), arg.password.as_ref().unwrap()) {
            return Err(Error::from("密码不正确!"));
        }
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
