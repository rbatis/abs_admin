use rbatis::plugin::page::{Page, PageRequest};
use crate::domain::domain::SysRole;
use crate::domain::dto::RolePageDTO;
use crate::dao::RB;
use rbatis::crud::CRUD;
use rbatis_core::Result;
///角色服务
pub struct SysRoleService {}

impl SysRoleService {

    ///角色分页
    pub async fn page(&self,arg: &RolePageDTO) -> Result<Page<SysRole>> {
        let w=RB.new_wrapper()
            .check()?;
        let data=RB.fetch_page_by_wrapper("",&w,&PageRequest::new(arg.page.unwrap_or(0),arg.size.unwrap_or(0))).await?;
        return Ok(data);
    }

}