use std::collections::BTreeMap;

use crate::domain::domain::{SysRes, SysUserRole};
use crate::domain::dto::{UserPageDTO, UserRoleAddDTO, UserRolePageDTO};
use crate::domain::vo::user::SysUserVO;
use crate::domain::vo::{SysResVO, SysRoleVO};
use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::crud::CRUD;
use rbatis::plugin::page::Page;
use rbatis::plugin::snowflake::new_snowflake_id;

///用户角色服务
pub struct SysUserRoleService {}

impl SysUserRoleService {
    ///角色分页
    pub async fn page(&self, arg: &UserRolePageDTO) -> Result<Page<SysUserVO>> {
        let mut vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(arg))
            .await?;
        if arg.resp_set_role.unwrap_or(true) {
            let all_role = CONTEXT.sys_role_service.finds_all_map().await?;
            let user_ids = rbatis::make_table_field_vec!(&vo.records, id);
            let user_roles = CONTEXT
                .rbatis
                .fetch_list_by_wrapper::<SysUserRole>(
                    CONTEXT.rbatis.new_wrapper().in_("user_id", &user_ids),
                )
                .await?;
            let user_role_map = rbatis::make_table_field_map!(&user_roles, user_id);
            let role_ids = rbatis::make_table_field_vec!(&user_roles, role_id);
            let roles = CONTEXT.sys_role_service.finds(&role_ids).await?;
            let roles_map = rbatis::make_table_field_map!(&roles, id);
            for mut x in &mut vo.records {
                let user_role = user_role_map.get(&x.id.clone().unwrap_or_default());
                match user_role {
                    Some(user_role) => {
                        match &user_role.role_id {
                            Some(role_id) => {
                                let role = roles_map.get(role_id).cloned();
                                x.role = SysRoleVO::from_option(role);
                                //查找子集角色
                                match &mut x.role {
                                    None => {}
                                    Some(role_vo) => {
                                        CONTEXT
                                            .sys_role_service
                                            .loop_find_childs(role_vo, &all_role);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        return Ok(vo);
    }

    ///角色添加
    pub async fn add(&self, arg: &UserRoleAddDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from("添加角色时用户和角色不能为空！"));
        }
        let mut role = SysUserRole {
            id: arg.id.clone(),
            user_id: arg.user_id.clone(),
            role_id: arg.role_id.clone(),
            create_date: NaiveDateTime::now().into(),
        };
        if role.id.is_none() {
            role.id = Some(new_snowflake_id().to_string());
        }
        self.remove_by_user_id(&arg.user_id.clone().unwrap_or_default())
            .await?;
        Ok(CONTEXT.rbatis.save(&role, &[]).await?.rows_affected)
    }

    ///角色删除
    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(CONTEXT
            .rbatis
            .remove_by_wrapper::<SysUserRole>(CONTEXT.rbatis.new_wrapper().eq("role_id", role_id))
            .await?)
    }

    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(CONTEXT
            .rbatis
            .remove_by_wrapper::<SysUserRole>(CONTEXT.rbatis.new_wrapper().eq("user_id", user_id))
            .await?)
    }

    ///找出角色
    pub async fn find_user_role(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysRes>,
    ) -> Result<Option<SysRoleVO>> {
        if user_id.is_empty() {
            return Ok(None);
        }
        let user_roles = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysUserRole>(
                CONTEXT.rbatis.new_wrapper().eq("user_id", user_id),
            )
            .await?;
        let role_ids = &rbatis::make_table_field_vec!(&user_roles, role_id);
        let roles = CONTEXT.sys_role_service.finds(role_ids).await?;
        let role_res_vec = CONTEXT
            .sys_role_service
            .find_role_res(&rbatis::make_table_field_vec!(&user_roles, role_id))
            .await?;

        let mut role_vos = vec![];
        for role in roles {
            //load res
            let mut resources = vec![];
            for role_res in &role_res_vec {
                if role.id.is_some() && role.id.eq(&role_res.role_id) {
                    let res = all_res.get(role_res.res_id.as_ref().unwrap_or(&String::new()));
                    match res {
                        Some(res) => {
                            resources.push(SysResVO::from(res));
                        }
                        _ => {}
                    }
                }
            }
            role_vos.push(SysRoleVO {
                id: role.id,
                name: role.name,
                parent_id: role.parent_id,
                del: role.del,
                create_date: role.create_date,
                resource_ids: CONTEXT.sys_res_service.make_res_ids(&resources),
                resources: resources,
                childs: None,
            });
        }
        if role_vos.is_empty() {
            return Ok(None);
        } else {
            return Ok(Some(role_vos[0].clone()));
        }
    }
}
