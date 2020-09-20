use rbatis::crud::{CRUD};
use rbatis::plugin::page::{Page, PageRequest};
use rbatis_core::Result;

use crate::dao::RB;
use crate::domain::domain::{SysRole, SysUserRole};
use crate::domain::dto::{RoleAddDTO, RoleEditDTO, RolePageDTO};

///角色服务
pub struct SysRoleService {}

impl SysRoleService {
    ///角色分页
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRole>> {
        let wrapper = RB.new_wrapper()
            .check()?;
        let data = RB.fetch_page_by_wrapper("", &wrapper, &PageRequest::new(arg.page.unwrap_or(0), arg.size.unwrap_or(0))).await?;
        return Ok(data);
    }

    ///角色添加
    pub async fn add(&self, arg: &RoleAddDTO) -> Result<u64> {
        unimplemented!();
    }

    ///角色修改
    pub async fn edit(&self, arg: &RoleEditDTO) -> Result<u64> {
        unimplemented!();
    }

    ///角色删除
    pub async fn remove(&self, arg: &str) -> Result<u64> {
        unimplemented!();
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        RB.list_by_wrapper("", RB.new_wrapper().in_array("id", ids)).await
    }

    pub async fn find_user_permission(&self, user_id: &str) -> Result<Vec<String>> {
        let user_roles: Vec<SysUserRole> = RB.list_by_wrapper("", RB.new_wrapper().eq("user_id", user_id)).await?;
        let mut role_ids = vec![];
        for item in &user_roles {
            role_ids.push(item.role_id.clone().unwrap_or("".to_string()));
        }
        let roles = self.finds(&role_ids).await?;

        unimplemented!()
    }
}