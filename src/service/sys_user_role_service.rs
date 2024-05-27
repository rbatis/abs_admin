use std::collections::HashMap;

use crate::domain::dto::{UserPageDTO, UserRoleAddDTO, UserRolePageDTO};
use crate::domain::table::SysUserRole;
use crate::domain::vo::user::SysUserVO;
use crate::domain::vo::{SysPermissionVO, SysRoleVO};
use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use crate::{error_info, pool};
use log::info;
use rbatis::Page;

#[derive(Default)]
///User Role Service
pub struct SysUserRoleService {}

impl SysUserRoleService {
    pub async fn page(&self, arg: &UserRolePageDTO) -> Result<Page<SysUserVO>> {
        let mut vo = CONTEXT
            .sys_user_service
            .page(&UserPageDTO::from(arg))
            .await?;
        if vo.records.is_empty() {
            return Ok(vo);
        }
        if arg.resp_set_role.unwrap_or(true) {
            // 作者的想法应该是:分页的用户id,一次查询用户的角色,然后组装用户角色映射,最后组装用户角色的角色映射
            let all_roles = CONTEXT.sys_role_service.finds_all_map().await?;
            // let roles = CONTEXT.sys_role_service.finds_all().await?;
            // let all_roles = rbatis::make_table_field_map!(&roles, id);
            info!("all_roles: {:?}", all_roles);
            let user_ids = rbatis::make_table_field_vec!(&vo.records, id);
            // 查询所有用户的角色
            let user_roles = SysUserRole::select_in_column(pool!(), "user_id", &user_ids).await?;
            info!("user_roles: {:?}", user_roles);
            let user_role_map = rbatis::make_table_field_map!(&user_roles, user_id);
            // let role_ids = rbatis::make_table_field_vec!(&user_roles, role_id);
            // // 又查询所有角色
            // let roles = CONTEXT.sys_role_service.finds(&role_ids).await?;
            // info!("roles: {:?}", roles);
            for x in &mut vo.records {
                if let Some(user_role) = user_role_map.get(x.id.as_deref().unwrap_or_default()) {
                    if let Some(role_id) = &user_role.role_id {
                        let role = all_roles.get(role_id).cloned();
                        x.role = SysRoleVO::from_option(role);
                        if let Some(role_vo) = &mut x.role {
                            CONTEXT
                                .sys_role_service
                                .loop_find_childs(role_vo, &all_roles);
                        }
                    }
                }
            }
        }
        Ok(vo)
    }

    pub async fn add(&self, arg: UserRoleAddDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from(error_info!("role_user_cannot_empty")));
        }
        // let user_id = arg.user_id.clone().unwrap();
        let role = SysUserRole::from(arg);
        // if role.id.is_none() {
            // role.id = Some(ObjectId::new().to_string());
        // }
        // 原来是先删除再添加
        // self.remove_by_user_id(&user_id).await?;
        let mut rows = SysUserRole::update_by_column(
            pool!(),
            &role,
            "user_id",
        ).await?.rows_affected;
        if rows == 0 {
            rows = SysUserRole::insert(pool!(), &role).await?.rows_affected;
        }
        
        Ok(rows)
    }

    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(SysUserRole::delete_by_column(pool!(), "role_id", role_id)
            .await?
            .rows_affected)
    }

    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(SysUserRole::delete_by_column(pool!(), "user_id", user_id)
            .await?
            .rows_affected)
    }

    pub async fn find_user_role(
        &self,
        user_id: &str,
        all_res: &HashMap<String, SysPermissionVO>,
    ) -> Result<Option<SysRoleVO>> {
        info!("find_user_role: {}", user_id);
        if user_id.is_empty() {
            return Ok(None);
        }
        let user_roles = SysUserRole::select_by_column(pool!(), "user_id", user_id).await?;
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
                    if let Some(res) =
                        all_res.get(role_res.permission_id.as_deref().unwrap_or_default())
                    {
                        resources.push(res.clone());
                    }
                }
            }
            let mut vo = SysRoleVO::from(role);
            vo.resource_ids = CONTEXT
                .sys_permission_service
                .make_permission_ids(&resources);
            vo.resources = resources;
            role_vos.push(vo);
        }
        if role_vos.is_empty() {
            Ok(None)
        } else {
            Ok(Some(role_vos[0].clone()))
        }
    }
}
