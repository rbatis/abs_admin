use crate::context::CONTEXT;
use crate::domain::dto::rbac::{
    RoleAddDTO, RoleEditDTO, RolePageDTO, SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO,
};
use crate::domain::table::rbac::RbacRolePermission;
use crate::domain::vo::rbac::SysRoleVO;
use crate::error::Error;
use crate::error::Result;
use crate::{error_info, pool};
use rbatis::plugin::object_id::ObjectId;
use rbatis::rbdc::DateTime;
use rbatis::Page;
use rbs::value;

/// Role Resource Service
pub struct RbacRolePermissionService {}

impl RbacRolePermissionService {
    pub async fn page(&self, arg: &SysRoleResPageDTO) -> Result<Page<SysRoleVO>> {
        let role_page = CONTEXT
            .rbac_role_service
            .page(&RolePageDTO {
                page_no: arg.page_no.clone(),
                page_size: arg.page_size.clone(),
                name: arg.name.clone(),
            })
            .await?;
        Ok(role_page)
    }

    pub async fn find_by_role_ids(
        &self,
        role_ids: &Vec<String>,
    ) -> Result<Vec<RbacRolePermission>> {
        if role_ids.is_empty(){
            return Ok(vec![]);
        }
        let datas = RbacRolePermission::select_by_map(pool!(), value! {"role_id":role_ids}).await?;
        Ok(datas)
    }

    pub async fn add(&self, arg: &SysRoleResAddDTO) -> Result<u64> {
        let (_, role_id) = CONTEXT
            .rbac_role_service
            .add(RoleAddDTO::from(arg.clone()))
            .await?;
        self.save_resources(&role_id, arg.permission_ids.clone())
            .await
    }

    pub async fn edit(&self, arg: &SysRoleResUpdateDTO) -> Result<u64> {
        let role_id = arg
            .id
            .as_ref()
            .ok_or_else(|| Error::from(error_info!("role_id_empty")))?;
        CONTEXT
            .rbac_role_service
            .edit(RoleEditDTO::from(arg.clone()))
            .await?;
        self.save_resources(role_id, arg.permission_ids.clone()).await
    }

    async fn save_resources(&self, role_id: &str, permission_ids: Vec<String>) -> Result<u64> {
        self.remove_by_role_id(role_id).await?;
        let mut sys_role_permission = Vec::with_capacity(permission_ids.len());
        for resource_id in permission_ids {
            sys_role_permission.push(RbacRolePermission {
                id: ObjectId::new().to_string().into(),
                role_id: role_id.to_string().into(),
                permission_id: resource_id.clone().into(),
                create_date: DateTime::now().into(),
            });
        }
        Ok(
            RbacRolePermission::insert_batch(pool!(), &sys_role_permission, 20)
                .await?
                .rows_affected,
        )
    }

    ///Roles, user relationships, and rights are deleted
    pub async fn remove_role(&self, role_id: &str) -> Result<u64> {
        let remove_roles = CONTEXT.rbac_role_service.remove(role_id).await?;
        let remove_user_roles = CONTEXT
            .rbac_user_role_service
            .remove_by_role_id(role_id)
            .await?;
        let remove_role_res = CONTEXT
            .rbac_role_permission_service
            .remove_by_role_id(role_id)
            .await?;
        Ok(remove_roles + remove_user_roles + remove_role_res)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        Ok(RbacRolePermission::delete_by_map(pool!(), value! {"id":id})
            .await?
            .rows_affected)
    }

    pub async fn remove_by_permission_id(&self, permission_id: &str) -> Result<u64> {
        Ok(
            RbacRolePermission::delete_by_map(pool!(), value! {"permission_id": permission_id})
                .await?
                .rows_affected,
        )
    }

    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(
            RbacRolePermission::delete_by_map(pool!(), value! {"role_id": role_id})
                .await?
                .rows_affected,
        )
    }
}
