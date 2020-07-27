use crate::domain::dto::SignInDTO;
use rbatis_core::Error;
use crate::dao::RB;
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use crate::domain::domain::BizAdminUser;
use crate::domain::vo::SignInVO;
use rbatis_core::Result;

///后台用户服务
pub struct AdminUserService {}

impl AdminUserService {
    ///登陆
    pub async fn sign_in(arg: &SignInDTO) -> Result<SignInVO> {
        let w = Wrapper::new(&RB.driver_type()?).eq("account", &arg.account).check()?;
        let u: BizAdminUser = RB.fetch_by_wrapper("", &w).await?;
        let signVO = SignInVO {
            user: Some(u),
            permissions: vec![],
        };
        //TODO load permission
        //TODO save redis cache
        return Ok(signVO);
    }

    ///登出
    pub async fn sign_out() {}
}
