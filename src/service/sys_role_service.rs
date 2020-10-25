use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis_core::Result;

use crate::dao::RB;
use crate::domain::domain::{SysRole, SysRoleRes, SysUserRole};
use crate::domain::dto::{RoleAddDTO, RoleEditDTO, RolePageDTO};
use crate::service::SYS_RES_SERVICE;
use uuid::Uuid;


///角色服务
pub struct SysRoleService {}

impl SysRoleService {
    ///角色分页
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRole>> {
        let wrapper = RB.new_wrapper()
            .check()?;
        let data = RB.fetch_page_by_wrapper("", &wrapper, &PageRequest::new(arg.page.unwrap_or(0), arg.size.unwrap_or(0))).await?;
        return Ok(data);
    }

    ///角色添加
    pub async fn add(&self, arg: &RoleAddDTO) -> Result<u64> {
        let role=SysRole{
            id: Some(Uuid::new_v4().to_string()),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: Some(0),
            create_time: None
        };
        RB.save("",&role).await
    }

    ///角色修改
    pub async fn edit(&self, arg: &RoleEditDTO) -> Result<u64> {
        let role=SysRole{
            id: arg.id.clone(),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: None,
            create_time: None
        };
        RB.update_by_id("",&role).await
    }

    ///角色删除
    pub async fn remove(&self, arg: &str) -> Result<u64> {
        RB.remove_by_id::<SysRole>("",&arg.to_string()).await
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        RB.list_by_wrapper("", &RB.new_wrapper().in_array("id", ids).check()?).await
    }

    pub async fn find_role_res(&self, ids: &Vec<String>) -> Result<Vec<SysRoleRes>> {
        RB.list_by_wrapper("", &RB.new_wrapper().in_array("role_id", ids).check()?).await
    }


    pub async fn find_user_permission(&self, user_id: &str) -> Result<Vec<String>> {
        let user_roles: Vec<SysUserRole> = RB.list_by_wrapper("", &RB.new_wrapper().eq("user_id", user_id).check()?).await?;
        let role_ids = to_field_vec!(&user_roles,role_id);
        let role_res = self.find_role_res(&role_ids).await?;
        let res_ids = to_field_vec!(&role_res,res_id);
        let all_res = SYS_RES_SERVICE.finds_all().await?;
        let res = SYS_RES_SERVICE.finds_layer(&res_ids, &all_res).await?;
        let mut permissons = vec![];
        for item in res {
            permissons.push(item.permission.clone().unwrap_or("".to_string()));
        }
        return Ok(permissons);
    }
}
