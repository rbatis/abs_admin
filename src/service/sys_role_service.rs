use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use crate::domain::domain::{SysRes, SysRole, SysRoleRes, SysUserRole};
use crate::domain::dto::{RoleAddDTO, RoleEditDTO, RolePageDTO};
use crate::service::CONTEXT;

///角色服务
pub struct SysRoleService {}

impl SysRoleService {
    ///角色分页
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRole>> {
        let wrapper = CONTEXT.rbatis.new_wrapper();
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper(
                "",
                &wrapper,
                &PageRequest::new(arg.page.unwrap_or(0), arg.size.unwrap_or(0)),
            )
            .await?;
        return Ok(data);
    }

    ///角色添加
    pub async fn add(&self, arg: &RoleAddDTO) -> Result<(u64, String)> {
        let role = SysRole {
            id: Some(
                rbatis::plugin::snowflake::async_snowflake_id()
                    .await
                    .to_string(),
            ),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: Some(0),
            create_date: Some(NaiveDateTime::now()),
        };
        Ok((
            CONTEXT.rbatis.save("", &role).await?.rows_affected,
            role.id.clone().unwrap(),
        ))
    }

    ///角色修改
    pub async fn edit(&self, arg: &RoleEditDTO) -> Result<u64> {
        let mut role = SysRole {
            id: arg.id.clone(),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: None,
            create_date: None,
        };
        CONTEXT.rbatis.update_by_id("", &mut role).await
    }

    ///角色删除
    pub async fn remove(&self, id: &str) -> Result<u64> {
        CONTEXT
            .rbatis
            .remove_by_id::<SysRole>("", &id.to_string())
            .await
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        CONTEXT
            .rbatis
            .fetch_list_by_wrapper("", &CONTEXT.rbatis.new_wrapper().r#in("id", ids))
            .await
    }

    pub async fn find_role_res(&self, ids: &Vec<String>) -> Result<Vec<SysRoleRes>> {
        CONTEXT
            .rbatis
            .fetch_list_by_wrapper("", &CONTEXT.rbatis.new_wrapper().r#in("role_id", ids))
            .await
    }

    pub async fn find_user_permission(
        &self,
        user_id: &str,
        all_res: &Vec<SysRes>,
    ) -> Result<Vec<String>> {
        let user_roles: Vec<SysUserRole> = CONTEXT
            .rbatis
            .fetch_list_by_wrapper("", &CONTEXT.rbatis.new_wrapper().eq("user_id", user_id))
            .await?;
        let role_res = self
            .find_role_res(&field_vec!(&user_roles, role_id))
            .await?;
        let res = CONTEXT
            .sys_res_service
            .finds_layer(&field_vec!(&role_res, res_id), &all_res)
            .await?;
        let permissons = field_vec!(&res, permission);
        return Ok(permissons);
    }
}
