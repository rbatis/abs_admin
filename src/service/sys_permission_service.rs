use crate::domain::dto::{ResEditDTO, ResPageDTO};
use crate::domain::table::SysPermission;
use crate::domain::vo::SysPermissionVO;
use crate::error::Error;
use crate::error::Result;
use crate::context::CONTEXT;
use crate::{error_info, pool};
use rbatis::{Page, PageRequest};
use std::collections::{BTreeMap, HashMap};

const RES_KEY: &'static str = "sys_permission:all";

/// Resource service
pub struct SysPermissionService {}

impl SysPermissionService {
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysPermissionVO>> {
        let data = SysPermission::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let all_res = self.finds_all_map().await?;
        let mut all_res_vo = HashMap::new();
        for (k, v) in all_res {
            all_res_vo.insert(k, v);
        }
        let mut page = Page::<SysPermissionVO>::from(data);
        for vo in &mut page.records {
            vo.set_childs_recursive(&all_res_vo);
        }
        Ok(page)
    }

    pub async fn add(&self, arg: &SysPermission) -> Result<u64> {
        let old = SysPermission::select_by_permission_or_name(
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
        let result = Ok(SysPermission::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        result
    }

    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        let data = SysPermission::from(arg);
        let result = SysPermission::update_by_column(pool!(), &data, "id").await?;
        self.update_cache().await?;
        Ok(result.rows_affected)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        let num = SysPermission::delete_by_column(pool!(), "id", id)
            .await?
            .rows_affected;
        SysPermission::delete_by_column(pool!(), "parent_id", id).await?;
        let _ = CONTEXT
            .sys_role_permission_service
            .remove_by_permission_id(id)
            .await;
        self.update_cache().await?;
        Ok(num)
    }

    pub fn make_permission_ids(&self, args: &Vec<SysPermissionVO>) -> Vec<String> {
        let mut ids = Vec::with_capacity({
            let mut cap = 0;
            for x in args {
                if let Some(childs) = &x.childs {
                    cap += childs.len();
                }
            }
            cap
        });
        for x in args {
            ids.push(x.id.as_deref().unwrap_or_default().to_string());
            if let Some(childs) = &x.childs {
                let child_ids = rbatis::table_field_vec!(childs, id);
                for child_id in child_ids {
                    ids.push(child_id);
                }
            }
        }
        ids
    }

    /// Find the res array
    pub async fn finds_all(&self) -> Result<Vec<SysPermissionVO>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysPermission>>>(RES_KEY)
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
        let mut arr = Vec::with_capacity({
            let mut cap = 0;
            if let Ok(v) = &js {
                if let Some(v) = v {
                    cap = v.len();
                }
            }
            cap
        });
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        Ok(arr)
    }

    pub async fn update_cache(&self) -> Result<Vec<SysPermissionVO>> {
        let all = SysPermission::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        let mut v = Vec::with_capacity(all.len());
        for x in all {
            v.push(x.into());
        }
        Ok(v)
    }

    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysPermissionVO>> {
        let all = self.finds_all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.id.as_deref().unwrap_or_default().to_string(), x);
        }
        Ok(result)
    }

    pub fn finds_res(
        &self,
        ids: &Vec<String>,
        all_res: &BTreeMap<String, SysPermissionVO>,
    ) -> Vec<SysPermissionVO> {
        let mut res = Vec::with_capacity(all_res.len());
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
    pub async fn finds_layer_top(&self) -> Result<Vec<SysPermissionVO>> {
        let list = SysPermission::select_by_parent_id_null(pool!()).await?;
        let all = self.finds_all_map().await?;
        self.finds_layer(&rbatis::table_field_vec!(list, id), &all)
            .await
    }

    ///An res array with a hierarchy
    pub async fn finds_layer(
        &self,
        ids: &Vec<String>,
        all_res: &BTreeMap<String, SysPermissionVO>,
    ) -> Result<Vec<SysPermissionVO>> {
        let res = self.finds_res(ids, &all_res);
        //find tops
        let mut tops = Vec::with_capacity(res.len());
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

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_childs(
        &self,
        arg: &mut SysPermissionVO,
        all_res: &BTreeMap<String, SysPermissionVO>,
    ) {
        let mut childs = Vec::with_capacity(all_res.len());
        for (_key, x) in all_res {
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
