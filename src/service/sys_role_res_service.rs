use crate::domain::domain::SysRoleRes;
use crate::domain::dto::{RoleAddDTO, SysRoleResAddDTO, SysRoleResUpdateDTO};
use crate::service::CONTEXT;
use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::crud::CRUD;

/// 角色资源服务
pub struct SysRoleResService {}

impl SysRoleResService {
    ///添加角色资源
    pub async fn add(&self, arg: &SysRoleResAddDTO) -> Result<u64> {
        let (_, role_id) = CONTEXT.sys_role_service.add(&arg.role).await?;
        return self.save_resources(&role_id, &arg.resource_ids).await;
    }

    pub async fn edit(&self, arg: &SysRoleResUpdateDTO) -> Result<u64> {
        let role_id = arg
            .role
            .id
            .as_ref()
            .ok_or_else(|| Error::from("角色id不能为空！"))?;
        CONTEXT.sys_role_service.edit(&arg.role).await?;
        return self.save_resources(role_id, &arg.resource_ids).await;
    }

    ///保存所以资源
    async fn save_resources(&self, role_id: &str, arg: &Option<Vec<String>>) -> Result<u64> {
        self.remove_by_role_id(role_id).await?;
        let mut num = 0;
        match &arg {
            Some(resource_ids) => {
                for resource_id in resource_ids {
                    let save_ok = CONTEXT
                        .rbatis
                        .save(
                            "",
                            &SysRoleRes {
                                id: Some(
                                    rbatis::plugin::snowflake::block_snowflake_id().to_string(),
                                ),
                                role_id: Some(role_id.to_string()),
                                res_id: Some(resource_id.clone()),
                                create_date: None,
                            },
                        )
                        .await;
                    if save_ok.is_ok() {
                        num += 1;
                    }
                }
            }
            _ => {}
        }
        return Ok(num);
    }

    ///角色删除,同时删除用户关系，权限关系
    pub async fn remove_role(&self, role_id: &str) -> Result<u64> {
        //删角色
        let remove_roles = CONTEXT.sys_role_service.remove(role_id).await?;
        //删除用户-角色
        let remove_user_roles = CONTEXT.sys_user_role_service.remove_by_role_id(role_id).await?;
        //删除角色-资源
        let remove_role_res = CONTEXT.sys_role_res_service.remove_by_role_id(role_id).await?;
        return Ok(remove_roles + remove_user_roles + remove_role_res);
    }

    ///删除角色资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        CONTEXT
            .rbatis
            .remove_by_id::<SysRoleRes>("", &id.to_string())
            .await
    }

    ///删除角色资源
    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        CONTEXT
            .rbatis
            .remove_by_wrapper::<SysRoleRes>(
                "",
                &CONTEXT.rbatis.new_wrapper().eq("role_id", role_id),
            )
            .await
    }
}
