use rbatis::plugin::page::Page;
use crate::domain::domain::BizRole;
use crate::domain::dto::RolePageDTO;

///角色服务
pub struct RoleService {}

impl RoleService {

    pub async fn page(&self,arg: &RolePageDTO) -> Page<BizRole> {
        unimplemented!()
    }

}