use crate::error::Result;
use rbdc::types::datetime::FastDateTime;
use crate::domain::domain::{SysRole, SysRoleRes, SysUserRole};
use crate::domain::dto::{RoleAddDTO, RoleEditDTO, RolePageDTO};
use crate::domain::vo::{SysResVO, SysRoleVO};
use crate::service::CONTEXT;
use crate::util::string::IsEmptyString;

use std::collections::{BTreeMap, HashMap};
use rbatis::plugin::object_id::ObjectId;
use rbatis::sql::{Page, PageRequest};
use crate::pool;


const RES_KEY: &'static str = "sys_role:all";

///角色服务
pub struct SysRoleService {}

impl SysRoleService {
    ///角色分页
    pub async fn page(&self, arg: &RolePageDTO) -> Result<Page<SysRoleVO>> {
        // let wrapper = CONTEXT
        //     .rbatis
        //     .new_wrapper()
        //     .eq(SysRole::del(), 0)
        //     .do_if(!arg.name.is_empty(), |w| w.like(SysRole::name(), &arg.name))
        //     .is_null(SysRole::parent_id())
        //     .order_by(false, &[SysRole::create_date()]);
        // let data = CONTEXT
        //     .rbatis
        //     .fetch_page_by_wrapper::<SysRole>(
        //         wrapper,
        //         &PageRequest::new(arg.page_no.unwrap_or(0), arg.page_size.unwrap_or(10)),
        //     )
        //     .await?;
        // let all_role = self.finds_all_map().await?;
        // let mut datas = vec![];
        // for x in data.records {
        //     let mut vo = SysRoleVO::from(x);
        //     self.loop_find_childs(&mut vo, &all_role);
        //     datas.push(vo);
        // }
        // let new_page = Page {
        //     records: datas,
        //     total: data.total,
        //     pages: data.pages,
        //     page_no: data.page_no,
        //     page_size: data.page_size,
        //     search_count: data.search_count,
        // };
        // Ok(new_page)
        todo!()
    }

    pub async fn finds_layer(&self) -> Result<Vec<SysRoleVO>> {
        let all = self.finds_all_map().await?;
        let mut data = vec![];
        for (k, v) in &all {
            if v.parent_id.is_none() {
                let mut top = SysRoleVO::from(v.clone());
                self.loop_find_childs(&mut top, &all);
                data.push(top);
            }
        }
        return Ok(data);
    }

    /// 查找role数组
    pub async fn finds_all(&self) -> Result<Vec<SysRole>> {
        //查找的全部数据缓存于Redis，同时 remove，edit方法调用时刷新redis缓存
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysRole>>>(RES_KEY)
            .await;
        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        if CONTEXT.config.debug {
            log::info!("[abs_admin] get from redis:{}", RES_KEY);
        }
        return Ok(js?.unwrap_or_default());
    }

    /// 更新所有
    pub async fn update_cache(&self) -> Result<Vec<SysRole>> {
        let all = SysRole::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        return Ok(all);
    }

    pub async fn finds_all_map(&self) -> Result<HashMap<String, SysRole>> {
        let all = self.finds_all().await?;
        let mut result = HashMap::with_capacity(all.capacity());
        for x in all {
            result.insert(x.id.as_deref().unwrap_or_default().to_string(), x);
        }
        return Ok(result);
    }

    ///角色添加
    pub async fn add(&self, arg: &RoleAddDTO) -> Result<(u64, String)> {
        let role = SysRole {
            id: ObjectId::new().to_string().into(),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: 0.into(),
            create_date: FastDateTime::now().set_micro(0).into(),
        };
        let result = (
            SysRole::insert(pool!(),&role).await?.rows_affected,
            role.id.clone().unwrap(),
        );
        self.update_cache().await?;
        Ok(result)
    }

    ///角色修改
    pub async fn edit(&self, arg: &RoleEditDTO) -> Result<u64> {
        let mut role = SysRole {
            id: arg.id.clone(),
            name: arg.name.clone(),
            parent_id: arg.parent_id.clone(),
            del: None,
            create_date: None,
        };
        let result = SysRole::update_by_column(pool!(),&role,SysRole::id()).await;
        self.update_cache().await?;
        Ok(result?.rows_affected)
    }

    ///角色删除
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let result = SysRole::delete_by_column(  pool!(), SysRole::id(), id).await;
        self.update_cache().await?;
        Ok(result?.rows_affected)
    }

    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRole>> {
        Ok( SysRole::select_list_by_ids(pool!(),ids).await? )
    }

    pub async fn find_role_res(&self, ids: &Vec<String>) -> Result<Vec<SysRoleRes>> {
        Ok( SysRoleRes::select_by_role_id(pool!(),ids).await? )
    }

    pub async fn find_user_permission(
        &self,
        user_id: &str,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<String>> {
        // let user_roles: Vec<SysUserRole> = CONTEXT
        //     .rbatis
        //     .fetch_list_by_wrapper(CONTEXT.rbatis.new_wrapper().eq(SysUserRole::user_id(), user_id))
        //     .await?;
        // let role_res = self
        //     .find_role_res(&rbatis::make_table_field_vec!(&user_roles, role_id))
        //     .await?;
        // let res = CONTEXT
        //     .sys_res_service
        //     .finds_layer(&rbatis::make_table_field_vec!(&role_res, res_id), &all_res)
        //     .await?;
        // let permissions = rbatis::make_table_field_vec!(&res, permission);
        // return Ok(permissions);
        todo!()
    }

    ///死循环找出父-子 关联关系数组
    pub fn loop_find_childs(&self, arg: &mut SysRoleVO, all: &HashMap<String, SysRole>) {
        let mut childs = vec![];
        for (key, x) in all {
            if x.parent_id.is_some() && x.parent_id.eq(&arg.id) {
                let mut item = SysRoleVO::from(x.clone());
                self.loop_find_childs(&mut item, all);
                childs.push(item);
            }
        }
        if !childs.is_empty() {
            arg.childs = Some(childs);
        }
    }
}
