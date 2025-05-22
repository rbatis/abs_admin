use std::collections::HashMap;
use crate::context::CONTEXT;
use crate::domain::dto::rbac::UserRoleAddDTO;
use crate::domain::table::rbac::{IntoMap, IntoMapVec, RbacUserRole};
use crate::domain::vo::rbac::RbacPermissionVO;
use crate::domain::vo::rbac::SysRoleVO;
use crate::error::Error;
use crate::error::Result;
use crate::{error_info, pool};
use rbatis::plugin::object_id::ObjectId;
use rbs::value;

pub struct SetUserVO {
    //this is user_id
    pub id: Option<String>,
    //set user role
    pub roles: Vec<SysRoleVO>,
}

///User Role Service
pub struct RbacUserRoleService {}

impl RbacUserRoleService {
    ///set to user list
    pub async fn set_roles(&self, records: &mut Vec<SetUserVO>) -> Result<()> {
        let user_ids = rbatis::table_field_vec!(&*records, id);
        let user_roles = RbacUserRole::select_by_map(pool!(), value! {"user_id": &user_ids}).await?;
        let role_ids = rbatis::table_field_vec!(&user_roles, role_id).into_iter().map(|v|v.to_string()).collect();
        let user_id_map = user_roles.into_map(|v|v.user_id.clone().unwrap_or_default());

        let role_perms = CONTEXT.rbac_role_permission_service.find_by_role_ids(&role_ids).await?;
        let perm_ids:Vec<String> = rbatis::table_field_vec!(&role_perms, permission_id).into_iter().map(|v|v.to_string()).collect();
        let perms = CONTEXT.rbac_permission_service.finds(perm_ids).await?.into_map(|v|v.id.clone().unwrap_or_default());
        let mut role_perms_map = HashMap::new();
        for x in &role_perms {
            role_perms_map.insert(x.role_id.clone().unwrap_or_default(), vec![]);
        }
        for x in role_perms {
            if let Some(v)=perms.get(&x.permission_id.clone().unwrap_or_default()){
                if let Some(vec) =role_perms_map.get_mut(x.role_id.as_deref().unwrap_or_default()){
                    vec.push(v.clone());
                }
            }
        }
        let role_map = CONTEXT.rbac_role_service.finds(&role_ids).await?.into_map(|v|v.id.clone().unwrap_or_default());
        for x in records {
            let user_id = x.id.as_deref().unwrap_or_default();
            let default_user_roles = vec![];
            let user_roles = user_id_map.get(user_id).unwrap_or(&default_user_roles);
            let mut roles= Vec::with_capacity(user_roles.len());
            for x in user_roles {
                let role_id = x.role_id.clone().unwrap_or_default();
                if let Some(v) = role_map.get(&role_id){
                    let mut role = SysRoleVO::from(v.clone());
                    if let Some(perms)= role_perms_map.get(&role_id){
                        let mut perm_vos = Vec::with_capacity(perms.len());
                        for x in perms {
                            perm_vos.push(RbacPermissionVO::from(x.clone()));
                        }
                        role.set_permissions(perm_vos);
                    }else{
                        role.set_permissions(vec![]);
                    }
                    roles.push(role);
                }
            }
            x.roles = roles;
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
        Ok(RbacUserRole::delete_by_map(pool!(), value! {"role_id": role_id})
            .await?
            .rows_affected)
    }

    pub async fn remove_by_user_id(&self, user_id: &str) -> Result<u64> {
        Ok(RbacUserRole::delete_by_map(pool!(), value! {"user_id": user_id})
            .await?
            .rows_affected)
    }

    pub async fn find_user_role(
        &self,
        user_id: &str
    ) -> Result<Vec<SysRoleVO>> {
        if user_id.is_empty() {
            return Ok(vec![]);
        }
        let mut role_vos =vec![];
        role_vos.push(SetUserVO{
            id: Some(user_id.to_string()),
            roles: vec![],
        });
        self.set_roles(&mut role_vos).await?;
        Ok(role_vos.remove(0).roles)
    }
}
