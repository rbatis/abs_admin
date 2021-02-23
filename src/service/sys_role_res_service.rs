use std::collections::HashMap;

use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::Page;

use crate::domain::domain::{SysRole, SysRoleRes};
use crate::domain::dto::{
    IdDTO, RolePageDTO, SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO,
};
use crate::domain::vo::{SysResVO, SysRoleVO};
use crate::service::CONTEXT;

/// 角色资源服务
pub struct SysRoleResService {}

impl SysRoleResService {
    ///角色-资源详情
    pub async fn detail(&self, arg: &IdDTO) -> Result<SysRoleVO> {
        let role = CONTEXT
            .sys_role_service
            .find(&arg.id.clone().unwrap_or_default())
            .await?;
        self.make_sys_role_vo(role).await
    }

    ///角色-资源 总体分页
    pub async fn page(&self, arg: &SysRoleResPageDTO) -> Result<Page<SysRole>> {
        let role_page = CONTEXT
            .sys_role_service
            .page(&RolePageDTO {
                page_no: arg.page_no.clone(),
                page_size: arg.page_size.clone(),
                name: arg.name.clone()
            })
            .await?;
        return Result::Ok(role_page);
    }

    // /// 直接转换角色数组  Vec<SysRole> -> Vec<SysRoleVO>
    // async fn make_sys_role_vos(&self, arg: Vec<SysRole>) -> Result<Vec<SysRoleVO>> {
    //     let role_ids = field_vec!(&arg, id);
    //     let role_res_vec = CONTEXT
    //         .rbatis
    //         .fetch_list_by_wrapper::<SysRoleRes>(
    //             "",
    //             &CONTEXT.rbatis.new_wrapper().r#in("role_id", &role_ids),
    //         )
    //         .await?;
    //     let resource_map = CONTEXT.sys_res_service.finds_all_map().await?;
    //     let mut role_res_map: HashMap<String, Vec<SysRoleRes>> = HashMap::new();
    //     for role_res in role_res_vec {
    //         let role_id = role_res.role_id.clone().unwrap_or_default();
    //         if role_res_map.get(&role_id).is_none() {
    //             let datas = vec![];
    //             role_res_map.insert(role_id.clone(), datas);
    //         }
    //         let sets = role_res_map.get_mut(&role_id).unwrap();
    //         //去重添加
    //         for x in sets.iter() {
    //             if x.id.eq(&role_res.id) {
    //                 continue;
    //             }
    //         }
    //         sets.push(role_res);
    //     }
    //     let mut data = vec![];
    //     for role in arg {
    //         let res_ids = role_res_map.get(role.id.as_ref().unwrap_or(&"".to_string()));
    //         let mut roles = vec![];
    //         match res_ids {
    //             Some(res_ids) => {
    //                 for x in res_ids {
    //                     match resource_map.get(x.res_id.as_ref().unwrap_or(&String::new())) {
    //                         Some(res) => {
    //                             let vo = SysResVO::from(res);
    //                             roles.push(vo);
    //                         }
    //                         _ => {}
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         }
    //         let vo = SysRoleVO {
    //             id: role.id.clone(),
    //             name: role.name.clone(),
    //             parent_id: role.parent_id.clone(),
    //             del: role.del.clone(),
    //             create_date: role.create_date.clone(),
    //             resources: roles,
    //         };
    //         data.push(vo);
    //     }
    //     return Ok(data);
    // }

    /// SysRole -> SysRoleVO
    async fn make_sys_role_vo(&self, arg: SysRole) -> Result<SysRoleVO> {
        let role_res_vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysRoleRes>(
                "",
                &CONTEXT.rbatis.new_wrapper().eq("role_id", &arg.id),
            )
            .await?;
        let resource_map = CONTEXT.sys_res_service.finds_all_map().await?;
        let mut role_res_map: HashMap<String, Vec<SysRoleRes>> = HashMap::new();
        for role_res in role_res_vec {
            let role_id = role_res.role_id.clone().unwrap_or_default();
            if role_res_map.get(&role_id).is_none() {
                let datas = vec![];
                role_res_map.insert(role_id.clone(), datas);
            }
            let sets = role_res_map.get_mut(&role_id).unwrap();
            //去重添加
            for x in sets.iter() {
                if x.id.eq(&role_res.id) {
                    continue;
                }
            }
            sets.push(role_res);
        }
        let res_ids = role_res_map.get(arg.id.as_ref().unwrap_or(&"".to_string()));
        let mut roles = vec![];
        match res_ids {
            Some(res_ids) => {
                for x in res_ids {
                    match resource_map.get(x.res_id.as_ref().unwrap_or(&String::new())) {
                        Some(res) => {
                            let vo = SysResVO::from(res);
                            roles.push(vo);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        let vo = SysRoleVO {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            del: arg.del,
            create_date: arg.create_date,
            resources: roles,
            childs: None
        };
        return Ok(vo);
    }

    ///添加角色资源
    pub async fn add(&self, arg: &SysRoleResAddDTO) -> Result<u64> {
        let (_, role_id) = CONTEXT.sys_role_service.add(&arg.role).await?;
        return self
            .save_resources(&role_id, arg.resource_ids.clone().unwrap_or(vec![]))
            .await;
    }

    pub async fn edit(&self, arg: &SysRoleResUpdateDTO) -> Result<u64> {
        let role_id = arg
            .role
            .id
            .as_ref()
            .ok_or_else(|| Error::from("角色id不能为空！"))?;
        CONTEXT.sys_role_service.edit(&arg.role).await?;
        return self
            .save_resources(role_id, arg.resource_ids.clone().unwrap_or(vec![]))
            .await;
    }

    ///保存所以资源
    async fn save_resources(&self, role_id: &str, resource_ids: Vec<String>) -> Result<u64> {
        self.remove_by_role_id(role_id).await?;
        let mut num = 0;
        for resource_id in resource_ids {
            let save_ok = CONTEXT
                .rbatis
                .save(
                    "",
                    &SysRoleRes {
                        id: Some(
                            rbatis::plugin::snowflake::async_snowflake_id()
                                .await
                                .to_string(),
                        ),
                        role_id: Some(role_id.to_string()),
                        res_id: Some(resource_id.clone()),
                        create_date: None,
                    },
                )
                .await;
            if save_ok.is_ok() {
                num += 1;
            }
        }
        return Ok(num);
    }

    ///角色删除,同时删除用户关系，权限关系
    pub async fn remove_role(&self, role_id: &str) -> Result<u64> {
        //删角色
        let remove_roles = CONTEXT.sys_role_service.remove(role_id).await?;
        //删除用户-角色
        let remove_user_roles = CONTEXT
            .sys_user_role_service
            .remove_by_role_id(role_id)
            .await?;
        //删除角色-资源
        let remove_role_res = CONTEXT
            .sys_role_res_service
            .remove_by_role_id(role_id)
            .await?;
        return Ok(remove_roles + remove_user_roles + remove_role_res);
    }

    ///删除角色资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        CONTEXT
            .rbatis
            .remove_by_id::<SysRoleRes>("", &id.to_string())
            .await
    }

    pub async fn remove_by_res_id(&self, res_id: &str) -> Result<u64> {
        CONTEXT
            .rbatis
            .remove_by_wrapper::<SysRoleRes>("", &CONTEXT.rbatis.new_wrapper().eq("res_id", res_id))
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
