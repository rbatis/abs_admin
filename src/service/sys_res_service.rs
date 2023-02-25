use crate::domain::dto::{ResEditDTO, ResPageDTO};
use crate::domain::table::SysRes;
use crate::domain::vo::SysResVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::service::CONTEXT;
use rbatis::sql::{Page, PageRequest};
use std::collections::{BTreeMap, HashMap};
const RES_KEY: &'static str = "sys_res:all";

/// Resource service
pub struct SysResService {}

impl SysResService {
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysResVO>> {
        let _page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
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

    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        let data = SysRes::from(arg);
        let result = SysRes::update_by_column(pool!(), &data, "id").await?;
        self.update_cache().await?;
        return Ok(result.rows_affected);
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        let trash = SysRes::select_by_column(pool!(), "id", id).await?;
        let num = SysRes::delete_by_column(pool!(), "id", id)
            .await?
            .rows_affected;
        CONTEXT.sys_trash_service.add("sys_res", &trash).await?;

        let trash = SysRes::select_by_column(pool!(), "parent_id", id).await?;
        SysRes::delete_by_column(pool!(), "parent_id", id).await?;
        CONTEXT.sys_trash_service.add("sys_res", &trash).await?;
        let _ = CONTEXT.sys_role_res_service.remove_by_res_id(id).await;
        self.update_cache().await?;
        return Ok(num);
    }

    pub fn make_res_ids(&self, args: &Vec<SysResVO>) -> Vec<String> {
        let mut ids = vec![];
        for x in args {
            ids.push(x.inner.id.as_deref().unwrap_or_default().to_string());
            if let Some(childs) = &x.childs {
                let child_ids = rbatis::make_table_field_vec!(childs, inner.id);
                for child_id in child_ids {
                    ids.push(child_id);
                }
            }
        }
        ids
    }

    /// Find the res array
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
            log::info!("[abs_admin] get from cache:{}", RES_KEY);
        }
        let mut arr = vec![];
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        return Ok(arr);
    }

    pub async fn update_cache(&self) -> Result<Vec<SysResVO>> {
        let all = SysRes::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        let mut v = vec![];
        for x in all {
            v.push(x.into());
        }
        return Ok(v);
    }

    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysResVO>> {
        let all = self.finds_all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.inner.id.as_deref().unwrap_or_default().to_string(), x);
        }
        return Ok(result);
    }

    pub fn finds_res(
        &self,
        ids: &Vec<String>,
        all_res: &BTreeMap<String, SysResVO>,
    ) -> Vec<SysResVO> {
        let mut res = vec![];
        //filter res id
        for x in ids {
            for (k, v) in all_res {
                if k.eq(x) {
                    res.push(v.clone());
                    break;
                }
            }
        }
        res
    }

    ///The top-level permissions
    pub async fn finds_layer_top(&self) -> Result<Vec<SysResVO>> {
        let list = SysRes::select_by_parent_id_null(pool!()).await?;
        let all = self.finds_all_map().await?;
        self.finds_layer(&rbatis::make_table_field_vec!(list, id), &all)
            .await
    }

    ///An res array with a hierarchy
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
            if item.inner.parent_id.is_none() {
                tops.push(item);
            }
        }
        //find child
        for mut item in &mut tops {
            self.loop_find_childs(&mut item, all_res);
        }
        Ok(tops)
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_childs(&self, arg: &mut SysResVO, all_res: &BTreeMap<String, SysResVO>) {
        let mut childs = vec![];
        for (_key, x) in all_res {
            if x.inner.parent_id.is_some() && x.inner.parent_id.eq(&arg.inner.id) {
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
