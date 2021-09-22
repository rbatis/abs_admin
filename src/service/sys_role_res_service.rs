use std::collections::{BTreeMap, HashMap};

use crate::error::Error;
use crate::error::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::Page;

use crate::domain::domain::{SysRes, SysRoleRes};
use crate::domain::dto::{
    RoleAddDTO, RoleEditDTO, RolePageDTO, SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO,
};
use crate::domain::vo::{SysResVO, SysRoleVO};
use crate::service::CONTEXT;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use rbatis::plugin::snowflake::new_snowflake_id;

/// 角色资源服务
pub struct SysRoleResService {}

impl SysRoleResService {
    ///角色-资源 总体分页
    pub async fn page(&self, arg: &SysRoleResPageDTO) -> Result<Page<SysRoleVO>> {
        let mut role_page = CONTEXT
            .sys_role_service
            .page(&RolePageDTO {
                page_no: arg.page_no.clone(),
                page_size: arg.page_size.clone(),
                name: arg.name.clone(),
            })
            .await?;
        let all = CONTEXT.sys_res_service.finds_all_map().await?;
        let role_res_map = self.find_role_res_map(&role_page.records).await?;
        role_page.records = self.loop_set_res_vec(role_page.records, &role_res_map, &all)?;
        return Result::Ok(role_page);
    }

    fn loop_find_role_ids(&self, arg: &Vec<SysRoleVO>) -> Vec<String> {
        let mut results = vec![];
        for x in arg {
            results.push(x.id.clone().unwrap_or_default());
            match &x.childs {
                Some(childs) => {
                    let ids = self.loop_find_role_ids(childs);
                    for id in ids {
                        results.push(id);
                    }
                }
                _ => {}
            }
        }
        return results;
    }

    async fn find_role_res_map(
        &self,
        arg: &Vec<SysRoleVO>,
    ) -> Result<HashMap<String, Vec<SysRoleRes>>> {
        let role_ids = self.loop_find_role_ids(arg);
        let role_res_vec = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysRoleRes>(
                CONTEXT.rbatis.new_wrapper().r#in("role_id", &role_ids),
            )
            .await?;
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
        return Ok(role_res_map);
    }

    /// 添加资源
    fn loop_set_res_vec(
        &self,
        arg: Vec<SysRoleVO>,
        role_res_map: &HashMap<String, Vec<SysRoleRes>>,
        all: &BTreeMap<String, SysRes>,
    ) -> Result<Vec<SysRoleVO>> {
        let mut data = vec![];
        for role in arg {
            let res_ids = role_res_map.get(role.id.as_ref().unwrap_or(&"".to_string()));
            let mut res_vos = vec![];
            match res_ids {
                Some(res_ids) => {
                    for x in res_ids {
                        match all.get(x.res_id.as_ref().unwrap_or(&String::new())) {
                            Some(res) => {
                                let vo = SysResVO::from(res);
                                res_vos.push(vo);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            let mut vo = SysRoleVO {
                id: role.id.clone(),
                name: role.name.clone(),
                parent_id: role.parent_id.clone(),
                del: role.del.clone(),
                create_date: role.create_date.clone(),
                resources: res_vos,
                childs: None,
                resource_ids: vec![],
            };
            if role.childs.is_some() {
                vo.childs = Some(self.loop_set_res_vec(
                    role.childs.unwrap_or(vec![]),
                    role_res_map,
                    all,
                )?);
            }
            vo.resource_ids = CONTEXT.sys_res_service.make_res_ids(&vo.resources);
            data.push(vo);
        }
        return Ok(data);
    }

    ///添加角色资源
    pub async fn add(&self, arg: &SysRoleResAddDTO) -> Result<u64> {
        let (_, role_id) = CONTEXT
            .sys_role_service
            .add(&RoleAddDTO::from(arg.clone()))
            .await?;
        return self
            .save_resources(&role_id, arg.resource_ids.clone())
            .await;
    }

    pub async fn edit(&self, arg: &SysRoleResUpdateDTO) -> Result<u64> {
        let role_id = arg
            .id
            .as_ref()
            .ok_or_else(|| Error::from("角色id不能为空！"))?;
        CONTEXT
            .sys_role_service
            .edit(&RoleEditDTO::from(arg.clone()))
            .await?;
        return self.save_resources(role_id, arg.resource_ids.clone()).await;
    }

    ///保存所以资源
    async fn save_resources(&self, role_id: &str, resource_ids: Vec<String>) -> Result<u64> {
        self.remove_by_role_id(role_id).await?;
        let mut sys_role_res = vec![];
        for resource_id in resource_ids {
            sys_role_res.push(SysRoleRes {
                id: new_snowflake_id().to_string().into(),
                role_id: role_id.to_string().into(),
                res_id: resource_id.clone().into(),
                create_date: NaiveDateTime::now().into(),
            });
        }
        let save_ok = CONTEXT.rbatis.save_batch(&sys_role_res, &[]).await?;
        return Ok(save_ok.rows_affected);
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
        Ok(CONTEXT
            .rbatis
            .remove_by_column::<SysRoleRes, _>("id", &id)
            .await?)
    }

    pub async fn remove_by_res_id(&self, res_id: &str) -> Result<u64> {
        Ok(CONTEXT
            .rbatis
            .remove_by_wrapper::<SysRoleRes>(CONTEXT.rbatis.new_wrapper().eq("res_id", res_id))
            .await?)
    }

    ///删除角色资源
    pub async fn remove_by_role_id(&self, role_id: &str) -> Result<u64> {
        Ok(CONTEXT
            .rbatis
            .remove_by_wrapper::<SysRoleRes>(CONTEXT.rbatis.new_wrapper().eq("role_id", role_id))
            .await?)
    }
}
