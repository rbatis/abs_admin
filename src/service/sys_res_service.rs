use rbatis::sql::{Page, PageRequest};
use std::collections::{BTreeMap, HashMap};

use crate::domain::dto::{ResEditDTO, ResPageDTO};
use crate::domain::table::SysRes;
use crate::domain::vo::SysResVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;
const RES_KEY: &'static str = "sys_res:all";

/// 资源服务
pub struct SysResService {}

impl SysResService {
    ///资源分页
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysResVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysRes::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let all_res = self.finds_all_map().await?;
        let mut all_res_vo = HashMap::new();
        for (k, v) in all_res {
            all_res_vo.insert(k, v);
        }
        let mut page = Page::<SysResVO>::from(data);
        for vo in &mut page.records {
            vo.set_childs_recursive(&all_res_vo);
        }
        Ok(page)
    }

    ///添加资源
    pub async fn add(&self, arg: &SysRes) -> Result<u64> {
        let old = SysRes::select_by_permission_or_name(
            pool!(),
            arg.permission.as_deref().unwrap_or_default(),
            arg.name.as_deref().unwrap_or_default(),
        )
        .await?;
        if old.len() > 0 {
            return Err(Error::from(format!(
                "权限已存在! 权限:{:?}",
                rbatis::make_table_field_vec!(old, name)
            )));
        }
        let result = Ok(SysRes::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        return result;
    }

    ///修改资源
    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        let data = SysRes {
            id: arg.id.clone(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            del: None,
            create_date: None,
        };
        let result = SysRes::update_by_column(pool!(), &data, "id").await?;
        self.update_cache().await?;
        return Ok(result.rows_affected);
    }

    ///删除资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let trash = SysRes::select_by_column(pool!(), "id", id).await?;
        let num = SysRes::delete_by_column(pool!(), "id", id)
            .await?
            .rows_affected;
        CONTEXT.sys_trash_service.add("sys_res", &trash).await?;

        let trash =
            SysRes::select_by_column(pool!(), rbatis::field_name!(SysRes.parent_id), id).await?;
        //删除父级为id的记录
        SysRes::delete_by_column(pool!(), field_name!(SysRes.parent_id), id).await?;
        CONTEXT.sys_trash_service.add("sys_res", &trash).await?;
        // //删除关联数据
        CONTEXT.sys_role_res_service.remove_by_res_id(id).await;
        self.update_cache().await?;
        return Ok(num);
    }

    pub fn make_res_ids(&self, args: &Vec<SysResVO>) -> Vec<String> {
        let mut ids = vec![];
        for x in args {
            ids.push(x.id.as_deref().unwrap_or_default().to_string());
            if let Some(childs) = &x.childs {
                let child_ids = rbatis::make_table_field_vec!(childs, id);
                for child_id in child_ids {
                    ids.push(child_id);
                }
            }
        }
        ids
    }

    /// 查找res数组
    pub async fn finds_all(&self) -> Result<Vec<SysResVO>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysRes>>>(RES_KEY)
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
        let mut arr = vec![];
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        return Ok(arr);
    }

    /// 更新所有
    pub async fn update_cache(&self) -> Result<Vec<SysResVO>> {
        let all = SysRes::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        let mut v = vec![];
        for x in all {
            v.push(x.into());
        }
        return Ok(v);
    }

    /// 查找res数组
    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysResVO>> {
        let all = self.finds_all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.id.as_deref().unwrap_or_default().to_string(), x);
        }
        return Ok(result);
    }

    /// 查找res数组
    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRes>> {
        let res = SysRes::select_by_ids(pool!(), ids).await?;
        Ok(res)
    }

    /// 查找res数组
    pub fn finds_res(
        &self,
        ids: &Vec<String>,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Vec<SysResVO> {
        let mut res = vec![];
        //filter res id
        for (k, v) in all_res {
            for x in ids {
                if k.eq(x) {
                    res.push(v.clone());
                }
            }
        }
        res
    }

    ///顶层权限
    pub async fn finds_layer_top(&self) -> Result<Vec<SysResVO>> {
        let list = SysRes::select_by_parent_id_null(pool!()).await?;
        let all = self.finds_all_map().await?;
        self.finds_layer(&rbatis::make_table_field_vec!(list, id), &all)
            .await
    }

    ///带有层级结构的 res数组
    pub async fn finds_layer(
        &self,
        ids: &Vec<String>,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Result<Vec<SysResVO>> {
        let res = self.finds_res(ids, &all_res);
        //find tops
        let mut tops = vec![];
        for item in res {
            //parent id null, it is an top resource
            if item.parent_id.is_none() {
                tops.push(item);
            }
        }
        //find child
        for mut item in &mut tops {
            self.loop_find_childs(&mut item, all_res);
        }
        Ok(tops)
    }

    ///死循环找出父-子 关联关系数组
    pub fn loop_find_childs(&self, arg: &mut SysResVO, all_res: &BTreeMap<String, SysResVO>) {
        let mut childs = vec![];
        for (key, x) in all_res {
            if x.parent_id.is_some() && x.parent_id.eq(&arg.id) {
                let mut item = x.clone();
                self.loop_find_childs(&mut item, all_res);
                childs.push(item);
            }
        }
        if !childs.is_empty() {
            arg.childs = Some(childs);
        }
    }
}
