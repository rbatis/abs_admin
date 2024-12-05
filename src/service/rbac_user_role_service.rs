use crate::context::CONTEXT;
use crate::domain::dto::rbac::UserRoleAddDTO;
use crate::domain::table::rbac::RbacUserRole;
use crate::domain::vo::rbac::SysPermissionVO;
use crate::domain::vo::rbac::SysRoleVO;
use crate::error::Error;
use crate::error::Result;
use crate::{error_info, pool};
use rbatis::plugin::object_id::ObjectId;
use std::collections::BTreeMap;

pub struct SetUserVO {
    //this is user_id
    pub id: Option<String>,
    //set user role
    pub role: Option<SysRoleVO>,
}

///User Role Service
pub struct SysUserRoleService {}

impl SysUserRoleService {
    ///set to user list
    pub async fn set_roles(&self, records: &mut Vec<SetUserVO>) -> Result<()> {
        let all_roles = CONTEXT.sys_role_service.finds_all_map().await?;
        let user_ids = rbatis::table_field_vec!(&*records, id);
        let user_roles = RbacUserRole::select_in_column(pool!(), "user_id", &user_ids).await?;
        let role_ids = rbatis::table_field_vec!(&user_roles, role_id)
            .iter()
            .map(|v| v.to_string())
            .collect();
        let user_role_map = rbatis::table_field_map!(user_roles, user_id);
        let roles = CONTEXT.sys_role_service.finds(&role_ids).await?;
        let roles_map = rbatis::table_field_map!(&roles, id);
        for x in records {
            if let Some(user_role) = user_role_map.get(x.id.as_deref().unwrap_or_default()) {
                if let Some(role_id) = &user_role.role_id {
                    let role = roles_map.get(role_id).cloned().cloned();
                    x.role = SysRoleVO::from_option(role);
                    if let Some(role_vo) = &mut x.role {
                        CONTEXT
                            .sys_role_service
                            .loop_find_childs(role_vo, &all_roles);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn add(&self, arg: UserRoleAddDTO) -> Result<u64> {
        if arg.user_id.is_none() || arg.role_id.is_none() {
            return Err(Error::from(error_info!("role_user_cannot_empty")));
        }
        let user_id = arg.user_id.as_deref().unwrap_or_default().to_string();
        let mut role = RbacUserRole::from(arg);
        if role.id.is_none() {
            role.id = Some(ObjectId::new().to_string());
        }
        self.remove_by_user_id(user_id.as_str()).await?;
        Ok(RbacUserRole::insert(pool!(), &role).await?.rows_affected)
    }

    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(RbacUserRole::delete_by_column(pool!(), "role_id", role_id)
            .await?
            .rows_affected)
    }

    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(RbacUserRole::delete_by_column(pool!(), "user_id", user_id)
            .await?
            .rows_affected)
    }

    pub async fn find_user_role(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysPermissionVO>,
    ) -> Result<Option<SysRoleVO>> {
        if user_id.is_empty() {
            return Ok(None);
        }
        let user_roles = RbacUserRole::select_by_column(pool!(), "user_id", user_id).await?;
        let role_ids = rbatis::table_field_vec!(user_roles, role_id);
        let roles = CONTEXT.sys_role_service.finds(&role_ids).await?;
        let role_res_vec = CONTEXT.sys_role_service.find_role_res(&role_ids).await?;
        let mut role_vos = Vec::with_capacity(roles.len());
        for role in roles {
            //load res
            let mut resources = Vec::with_capacity(role_res_vec.len());
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
