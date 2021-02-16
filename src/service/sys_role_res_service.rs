use crate::domain::domain::SysRoleRes;
use crate::service::CONTEXT;
use rbatis::core::Result;
use rbatis::crud::CRUD;

/// 角色资源服务
pub struct SysRoleResService {}

impl SysRoleResService {
    ///添加角色资源
    pub async fn add(&self, arg: SysRoleRes) -> Result<u64> {
        let old: Option<SysRoleRes> = CONTEXT
            .rbatis
            .fetch_by_wrapper(
                "",
                &CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq("role_id", &arg.role_id)
                    .eq("res_id", &arg.res_id),
            )
            .await?;
        if old.is_some() {
            //已存在
            return Ok(1);
        }
        Ok(CONTEXT.rbatis.save("", &arg).await?.rows_affected)
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
