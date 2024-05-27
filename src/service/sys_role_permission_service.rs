#![allow(clippy::only_used_in_recursion)]
use std::collections::{HashMap, HashSet};

use crate::domain::dto::{
    RoleAddDTO, RoleEditDTO, RolePageDTO, SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO,
};
use crate::domain::table::SysRolePermission;
use crate::domain::vo::{SysPermissionVO, SysRoleVO};
use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use crate::{error_info, pool};
use rbatis::plugin::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::Page;

#[derive(Default)]
/// Role Resource Service
pub struct SysRoleResService {}

impl SysRoleResService {
    pub async fn page(&self, arg: &SysRoleResPageDTO) -> Result<Page<SysRoleVO>> {
        let mut role_page = CONTEXT
            .sys_role_service
            .page(&RolePageDTO {
                page_no: arg.page_no,
                page_size: arg.page_size,
                name: arg.name.clone(),
            })
            .await?;
        let all = CONTEXT.sys_permission_service.finds_all_map().await?;
        let role_res = self.find_role_res(&role_page.records).await?;
        role_page.records = self.loop_set_res_vec(role_page.records, &role_res, &all)?;
        Result::Ok(role_page)
    }

    fn loop_find_role_ids(&self, arg: &Vec<SysRoleVO>) -> Vec<String> {
        let mut results = vec![];
        for x in arg {
            results.push(x.id.as_deref().unwrap_or_default().to_string());
            if let Some(childs) = &x.childs {
                let ids = self.loop_find_role_ids(childs);
                for id in ids {
                    results.push(id);
                }
            } 
        }
        results
    }

    async fn find_role_res(
        &self,
        arg: &Vec<SysRoleVO>,
    ) -> Result<Vec<SysRolePermission>> {
        let role_ids = self.loop_find_role_ids(arg);
        let role_res_vec = {
            if role_ids.is_empty() {
                vec![]
            } else {
                SysRolePermission::select_in_column(pool!(), "role_id", &role_ids).await?
            }
        };
        
        Ok(role_res_vec)
    }

    #[allow(dead_code)]
    async fn find_role_res_map(
        &self,
        arg: &Vec<SysRoleVO>,
    ) -> Result<HashMap<String, HashSet<SysRolePermission>>> {
        let role_ids = self.loop_find_role_ids(arg);
        let role_res_vec = {
            if role_ids.is_empty() {
                vec![]
            } else {
                SysRolePermission::select_in_column(pool!(), "role_id", &role_ids).await?
            }
        };
        let mut role_res_map: HashMap<String, HashSet<SysRolePermission>> =
            HashMap::with_capacity(role_res_vec.capacity());
        for role_res in role_res_vec {
            let role_id = role_res.role_id.as_deref().unwrap_or_default();
            //remove repeat
            match role_res_map.get_mut(role_id) {
                None => {
                    let role_id = role_id.to_string();
                    let mut sets = HashSet::new();
                    sets.insert(role_res);
                    role_res_map.insert(role_id, sets);
                }
                Some(sets) => {
                    sets.insert(role_res);
                }
            }
        }
        Ok(role_res_map)
    }

    /// Add the resource for role
    fn loop_set_res_vec(
        &self,
        arg: Vec<SysRoleVO>,
        role_res: &Vec<SysRolePermission>,
        all: &HashMap<String, SysPermissionVO>,
    ) -> Result<Vec<SysRoleVO>> {
        let mut data = vec![];
        for mut role in arg {
           
            let permission_ids: Vec<&SysRolePermission> = role_res.iter().filter(|x| x.role_id == role.id).collect();
            let mut res_vos = vec![];
            for x in permission_ids {
                if let Some(res) = all.get(x.permission_id.as_deref().unwrap_or_default()) {
                    res_vos.push(res.clone());
                }
            }

            role.resources = res_vos;
            if role.childs.is_some() {
                role.childs = Some(self.loop_set_res_vec(
                    role.childs.unwrap_or(vec![]),
                    role_res,
                    all,
                )?);
            }
            role.resource_ids = rbatis::make_table_field_vec!(&role.resources, id);
            data.push(role);
        }
        Ok(data)
    }

    
    pub async fn add(&self, arg: &SysRoleResAddDTO) -> Result<u64> {
        let (_, role_id) = CONTEXT
            .sys_role_service
            .add(RoleAddDTO::from(arg.clone()))
            .await?;
        self
            .save_resources(&role_id, arg.resource_ids.clone())
            .await
    }

    pub async fn edit(&self, arg: &SysRoleResUpdateDTO) -> Result<u64> {
        let role_id = arg
            .id
            .as_ref()
            .ok_or_else(|| Error::from(error_info!("role_id_empty")))?;
        CONTEXT
            .sys_role_service
            .edit(RoleEditDTO::from(arg.clone()))
            .await?;
        self.save_resources(role_id, arg.resource_ids.clone()).await
    }

    async fn save_resources(&self, role_id: &str, resource_ids: Vec<String>) -> Result<u64> {
        self.remove_by_role_id(role_id).await?;
        let mut sys_role_permission = Vec::with_capacity(resource_ids.len());
        for resource_id in resource_ids {
            sys_role_permission.push(SysRolePermission {
                id: ObjectId::new().to_string().into(),
                role_id: role_id.to_string().into(),
                permission_id: resource_id.clone().into(),
                create_date: DateTime::now().into(),
            });
        }
        Ok(
            SysRolePermission::insert_batch(pool!(), &sys_role_permission, 20)
                .await?
                .rows_affected,
        )
    }

    ///Roles, user relationships, and rights are deleted
    pub async fn remove_role(&self, role_id: &str) -> Result<u64> {
        let remove_roles = CONTEXT.sys_role_service.remove(role_id).await?;
        let remove_user_roles = CONTEXT
            .sys_user_role_service
            .remove_by_role_id(role_id)
            .await?;
        let remove_role_res = CONTEXT
            .sys_role_permission_service
            .remove_by_role_id(role_id)
            .await?;
        Ok(remove_roles + remove_user_roles + remove_role_res)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        Ok(SysRolePermission::delete_by_column(pool!(), "id", id)
            .await?
            .rows_affected)
    }

    pub async fn remove_by_permission_id(&self, permission_id: &str) -> Result<u64> {
        Ok(
            SysRolePermission::delete_by_column(pool!(), "permission_id", permission_id)
                .await?
                .rows_affected,
        )
    }

    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(
            SysRolePermission::delete_by_column(pool!(), "role_id", role_id)
                .await?
                .rows_affected,
        )
    }
}
