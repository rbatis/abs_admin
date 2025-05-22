use crate::context::CONTEXT;
use crate::domain::dto::rbac::{RoleAddDTO, RoleEditDTO, RolePageDTO};
use crate::domain::table::rbac::{RbacPermission, RbacRole, RbacRolePermission};
use crate::domain::vo::rbac::RbacPermissionVO;
use crate::domain::vo::rbac::SysRoleVO;
use crate::error::Result;
use crate::pool;
use rbatis::{Page, PageRequest};
use std::collections::{HashMap, HashSet};
use rbs::value;
use crate::domain::table::rbac::IntoMap;

///Role of service
pub struct RbacRoleService {}

impl RbacRoleService {
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        let data = RbacRole::select_page_by_name(
            pool!(),
            &PageRequest::from(arg),
            arg.name.as_deref().unwrap_or_default(),
        )
            .await?;
        let role_ids: Vec<String> = rbatis::table_field_set!(&data.records, id)
            .iter()
            .map(|v| v.to_string())
            .collect();
        let role_perms = CONTEXT
            .rbac_role_permission_service
            .find_by_role_ids(&role_ids)
            .await?;
        let perm_ids: Vec<String> = rbatis::table_field_set!(&role_perms, permission_id)
            .iter()
            .map(|v| v.to_string())
            .collect();
        let perm_map = CONTEXT.rbac_permission_service.finds(perm_ids).await?.into_map(|v|v.id.clone().unwrap_or_default());
        let role_perms = {
            let mut map = HashMap::<String, HashSet<RbacPermission>>::new();
            for x in role_perms {
                let role_id = x.role_id.clone().unwrap_or_default();
                if !map.contains_key(&role_id) {
                    map.insert(role_id.clone(), HashSet::new());
                }
                if let Some(role_perms) = map.get_mut(&role_id) {
                    if let Some(v) = perm_map.get(x.permission_id.as_deref().unwrap_or_default()) {
                        role_perms.insert(v.clone());
                    }
                }
            }
            map
        };
        let mut page = Page::<SysRoleVO>::from(data);
        for vo in &mut page.records {
            if let Some(perms) = role_perms.get(vo.id.as_deref().unwrap_or_default()) {
                vo.set_permissions(perms.iter().map(|v| RbacPermissionVO::from(v.clone())).collect());
            }
        }
        Ok(page)
    }


    pub async fn add(&self, arg: RoleAddDTO) -> Result<(u64, String)> {
        let role = RbacRole::from(arg);
        let result = (
            RbacRole::insert(pool!(), &role).await?.rows_affected,
            role.id.clone().unwrap_or_default(),
        );
        Ok(result)
    }

    pub async fn edit(&self, arg: RoleEditDTO) -> Result<u64> {
        let role = RbacRole::from(arg);
        let result = RbacRole::update_by_map(pool!(), &role, value! {"id": &role.id}).await;
        Ok(result?.rows_affected)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        let result = RbacRole::delete_by_map(pool!(), value! {"id": id}).await?;
        Ok(result.rows_affected)
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<RbacRole>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(RbacRole::select_by_map(pool!(), value! {"id":ids}).await?)
    }

    pub async fn find_role_res(&self, role_ids: &Vec<String>) -> Result<Vec<RbacRolePermission>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(RbacRolePermission::select_by_map(pool!(), value! {"role_id":role_ids}).await?)
    }

    pub async fn find_all(&self) -> Result<Vec<RbacRole>> {
        Ok(RbacRole::select_all(pool!()).await?)
    }
}
