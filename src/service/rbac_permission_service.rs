use crate::domain::dto::rbac::{ResEditDTO, PermissionPageDTO};
use crate::domain::table::rbac::RbacPermission;
use crate::domain::vo::rbac::RbacPermissionVO;
use crate::error::Error;
use crate::error::Result;
use crate::context::CONTEXT;
use crate::{error_info, pool};
use rbatis::{Page, PageRequest};
use rbs::value;

/// Resource service
pub struct RbacPermissionService {}

impl RbacPermissionService {
    pub async fn page(&self, arg: &PermissionPageDTO) -> Result<Page<RbacPermissionVO>> {
        let data = RbacPermission::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let page = Page::<RbacPermissionVO>::from(data);
        Ok(page)
    }

    pub async fn add(&self, arg: &RbacPermission) -> Result<u64> {
        let old = RbacPermission::select_by_permission_or_name(
            pool!(),
            arg.permission.as_deref().unwrap_or_default(),
            arg.name.as_deref().unwrap_or_default(),
        )
            .await?;
        if old.len() > 0 {
            return Err(Error::from(format!(
                "{}={:?}",
                error_info!("permission_exists"),
                rbatis::table_field_vec!(old, name)
            )));
        }
        let result = Ok(RbacPermission::insert(pool!(), &arg).await?.rows_affected);
        result
    }

    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        let data = RbacPermission::from(arg);
        let result = RbacPermission::update_by_map(pool!(), &data, value! {"id": &data.id }).await?;
        Ok(result.rows_affected)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        let num = RbacPermission::delete_by_map(pool!(), value! {"id":id})
            .await?
            .rows_affected;
        RbacPermission::delete_by_map(pool!(), value! {"id":id}).await?;
        let _ = CONTEXT
            .rbac_role_permission_service
            .remove_by_permission_id(id)
            .await;
        Ok(num)
    }

    pub async fn finds(&self,ids:Vec<String>) -> Result<Vec<RbacPermission>> {
        if ids.is_empty(){
            return Ok(vec![]);
        }
        let data=RbacPermission::select_by_map(pool!(), value! {"id": &ids}).await?;
        Ok(data)
    }

    pub async fn finds_all(&self) -> Result<Vec<RbacPermission>> {
        let data=RbacPermission::select_all(pool!()).await?;
        Ok(data)
    }
}
