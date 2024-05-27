#![allow(clippy::only_used_in_recursion)]
use crate::domain::dto::{ResEditDTO, ResPageDTO};
use crate::domain::table::SysPermission;
use crate::domain::vo::SysPermissionVO;
use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use crate::{error_info, pool};
use rbatis::{Page, PageRequest};
use std::collections::HashMap;
const RES_KEY: &str = "sys_permission:all";

/// Resource service
#[derive(Default)]
pub struct SysPermissionService {}

impl SysPermissionService {
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysPermissionVO>> {
        let data = SysPermission::select_page(pool!(), &PageRequest::from(arg), arg).await?;
        let all_res = self.finds_all_map().await?;
        // let mut all_res_vo = HashMap::new();
        // for (k, v) in all_res {
        //     all_res_vo.insert(k, v);
        // }
        let mut page = Page::<SysPermissionVO>::from(data);
        for vo in &mut page.records {
            vo.set_childs_recursive(&all_res);
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
        if !old.is_empty() {
            return Err(Error::from(format!(
                "{}={:?}",
                error_info!("permission_exists"),
                rbatis::make_table_field_vec!(old, name)
            )));
        }
        let result = Ok(SysPermission::insert(pool!(), arg).await?.rows_affected);
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

    /// collect all permission ids from res array and its childs
    pub fn make_permission_ids(&self, args: &[SysPermissionVO]) -> Vec<String> {
        let childs = args.iter().filter_map(|x| x.childs.as_ref()).flatten();
        let ids: Vec<String> = args.iter().chain(childs)
            .filter_map(|x| x.id.clone()).collect();
        ids
    }

    /// Find the res array
    pub async fn finds_all_vo(&self) -> Result<Vec<SysPermissionVO>> {
        let js = self.finds_all_cache().await?;
        let arr = js.into_iter().map(|x| x.into()).collect();
        
        Ok(arr)
    }

    pub async fn finds_all_cache(&self) -> Result<Vec<SysPermission>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysPermission>>>(RES_KEY)
            .await;
        let js = js.unwrap_or(None);
        if js.is_none() || js.as_ref().unwrap().is_empty()
        {
            return self.update_cache().await;
        }
        if CONTEXT.config.debug {
            log::info!("get from cache:{}", RES_KEY);
        }
        let arr = js.unwrap();
        Ok(arr)
    }

    pub async fn update_cache(&self) -> Result<Vec<SysPermission>> {
        log::info!("update cache: {}", RES_KEY);
        let all = SysPermission::select_all(pool!()).await?;
        CONTEXT.cache_service.set_json(RES_KEY, &all).await?;
        Ok(all)
    }

    pub async fn finds_all_map(&self) -> Result<HashMap<String, SysPermissionVO>> {
        let all = self.finds_all_vo().await?;
        // let result: BTreeMap<String, SysPermissionVO> = all.into_iter().map(|x| (x.id.clone().unwrap(), x)).collect();
        let result: HashMap<String, SysPermissionVO> = all.into_iter().map(|x| (x.id.clone().unwrap(), x)).collect();
        Ok(result)
    }

    pub fn finds_res(
        &self,
        ids: &[String],
        all_res: &HashMap<String, SysPermissionVO>,
    ) -> Vec<SysPermissionVO> {
        //filter res id
        let res = ids.iter().filter_map(|x| all_res.get(x)).cloned().collect::<Vec<SysPermissionVO>>();
        res
    }

    ///The top-level permissions
    pub async fn finds_layer_top(&self) -> Result<Vec<SysPermissionVO>> {
        let list = SysPermission::select_by_parent_id_null(pool!()).await?;
        let all = self.finds_all_map().await?;
        self.finds_layer(&rbatis::make_table_field_vec!(list, id), &all)
            .await
    }

    ///An res array with a hierarchy
    pub async fn finds_layer(
        &self,
        ids: &[String],
        all_res: &HashMap<String, SysPermissionVO>,
    ) -> Result<Vec<SysPermissionVO>> {
        let res = self.finds_res(ids, all_res);
        //find tops
        let mut tops = res.into_iter().filter(|x| x.parent_id.is_none()).collect::<Vec<SysPermissionVO>>();
        //find child
        for item in &mut tops {
            self.loop_find_childs(item, all_res);
        }
        Ok(tops)
    }

    ///Loop to find the parent-child associative relation array
    pub fn loop_find_childs(
        &self,
        arg: &mut SysPermissionVO,
        all_res: &HashMap<String, SysPermissionVO>,
    ) {
        let mut childs = vec![];
        for x in all_res.values() {
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
