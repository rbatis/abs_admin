use rbatis::plugin::page::Page;
use crate::domain::domain::SysRole;
use crate::domain::dto::RolePageDTO;

///角色服务
pub struct SysRoleService {}

impl SysRoleService {

    pub async fn page(&self,arg: &RolePageDTO) -> Page<SysRole> {
        unimplemented!()
    }

}