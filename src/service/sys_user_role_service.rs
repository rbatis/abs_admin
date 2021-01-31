use crate::dao::RB;
use crate::domain::domain::{SysUserRole};
use crate::domain::dto::{UserRoleAddDTO, UserRoleEditDTO, UserRolePageDTO};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use crate::service::{SYS_ROLE_SERVICE};
use crate::domain::vo::SysRoleVO;

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
        let mut role = SysUserRole {
            id: arg.id.clone(),
            user_id: arg.user_id.clone(),
            role_id: arg.role_id.clone(),
            create_date: None,
        };
        RB.update_by_id("", &mut role).await
    }

    ///角色删除
    pub async fn remove(&self, arg: &str) -> Result<u64> {
        RB.remove_by_id::<SysUserRole>("", &arg.to_string()).await
    }


    pub async fn find_user_roles(&self, user_id: &str) -> Result<Vec<SysRoleVO>> {
        let user_roles: Vec<SysUserRole> = RB
            .list_by_wrapper("", &RB.new_wrapper().eq("user_id", user_id))
            .await?;
        let role_ids=&to_field_vec!(&user_roles, role_id);
        let roles=SYS_ROLE_SERVICE.finds(role_ids).await?;
        let mut role_vos =vec![];
        for x in roles {
            role_vos.push(SysRoleVO{
                id: x.id,
                name: x.name,
                parent_id: x.parent_id,
                del: x.del,
                create_date: x.create_date,
                resources: vec![]
            });
        }
        return Ok(role_vos);
    }
}
