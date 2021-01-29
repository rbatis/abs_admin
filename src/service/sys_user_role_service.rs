use crate::dao::RB;
use crate::domain::domain::SysUserRole;
use crate::domain::dto::{UserRoleAddDTO, UserRoleEditDTO, UserRolePageDTO};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

///用户角色服务
pub struct SysUserRoleService {}

impl SysUserRoleService {
    ///角色分页
    pub async fn page(&self, arg: &UserRolePageDTO) -> Result<Page<SysUserRole>> {
        let wrapper = RB.new_wrapper();
        let data = RB
            .fetch_page_by_wrapper(
                "",
                &wrapper,
                &PageRequest::new(arg.page.unwrap_or(0), arg.size.unwrap_or(0)),
            )
            .await?;
        return Ok(data);
    }

    ///角色添加
    pub async fn add(&self, arg: &UserRoleAddDTO) -> Result<u64> {
        let role = SysUserRole {
            id: Some(
                rbatis::plugin::snowflake::async_snowflake_id()
                    .await
                    .to_string(),
            ),
            user_id: arg.user_id.clone(),
            role_id: arg.role_id.clone(),
            create_date: Some(NaiveDateTime::now()),
        };
        Ok(RB.save("", &role).await?.rows_affected)
    }

    ///角色修改
    pub async fn edit(&self, arg: &UserRoleEditDTO) -> Result<u64> {
        let role = SysUserRole {
            id: arg.id.clone(),
            user_id: arg.user_id.clone(),
            role_id: arg.role_id.clone(),
            create_date: None,
        };
        RB.update_by_id("", &role).await
    }

    ///角色删除
    pub async fn remove(&self, arg: &str) -> Result<u64> {
        RB.remove_by_id::<SysUserRole>("", &arg.to_string()).await
    }
}
