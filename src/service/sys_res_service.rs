use std::collections::{BTreeMap, HashMap};

use rbatis::crud::{Skip, CRUD};
use rbatis::plugin::page::{Page, PageRequest};

use crate::domain::domain::SysRes;
use crate::domain::dto::{ResEditDTO, ResPageDTO};
use crate::domain::vo::SysResVO;
use crate::error::Error;
use crate::error::Result;
use crate::service::cache_service::ICacheService;
use crate::service::CONTEXT;
use crate::util::string::IsEmptyString;

const RES_KEY: &'static str = "sys_res:all";

/// 资源服务
pub struct SysResService {}

impl SysResService {
    ///资源分页
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysResVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysRes>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysRes::del(), 0)
                    .do_if(!arg.name.is_empty(), |w| w.like(SysRes::name(), &arg.name))
                    .is_null(SysRes::parent_id())
                    .order_by(false, &[SysRes::create_date()]),
                &page_req,
            )
            .await?;
        let all_res = self.finds_all_map().await?;
        let mut all_res_vo = HashMap::new();
        for (k, v) in all_res {
            all_res_vo.insert(k, v);
        }
        let mut datas = vec![];
        for x in data.records {
            let mut vo = SysResVO::from(&x);
            vo.set_childs_recursive(&all_res_vo);
            datas.push(vo);
        }
        let new_page = Page {
            records: datas,
            total: data.total,
            pages: data.pages,
            page_no: data.page_no,
            page_size: data.page_size,
            search_count: data.search_count,
        };
        Ok(new_page)
    }

    ///添加资源
    pub async fn add(&self, arg: &SysRes) -> Result<u64> {
        let old: Vec<SysRes> = CONTEXT
            .rbatis
            .fetch_list_by_wrapper(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq(SysRes::permission(), &arg.permission)
                    .or()
                    .eq(SysRes::name(), &arg.name),
            )
            .await?;
        if old.len() > 0 {
            return Err(Error::from(format!(
                "权限已存在! 权限:{:?}",
                rbatis::make_table_field_vec!(old, name)
            )));
        }
        let result = Ok(CONTEXT.rbatis.save(arg, &[]).await?.rows_affected);
        self.update_cache().await?;
        return result;
    }

    ///修改资源
    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        let mut data = SysRes {
            id: arg.id.clone(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            permission: arg.permission.clone(),
            path: arg.path.clone(),
            del: None,
            create_date: None,
        };
        let result = Ok(CONTEXT
            .rbatis
            .update_by_wrapper(
                &mut data,
                CONTEXT.rbatis.new_wrapper().eq(SysRes::id(), &arg.id),
                &[
                    Skip::Column(SysRes::del()),
                    Skip::Column(SysRes::id()),
                    Skip::Column(SysRes::create_date()),
                ],
            )
            .await?);
        self.update_cache().await?;
        return result;
    }

    ///删除资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let num = CONTEXT
            .rbatis
            .remove_by_column::<SysRes, _>(SysRes::id(), &id.to_string())
            .await?;
        //删除父级为id的记录
        CONTEXT
            .rbatis
            .remove_by_wrapper::<SysRes>(CONTEXT.rbatis.new_wrapper().eq(SysRes::parent_id(), id))
            .await;
        //删除关联数据
        CONTEXT.sys_role_res_service.remove_by_res_id(id).await;
        self.update_cache().await?;
        return Ok(num);
    }

    pub fn make_res_ids(&self, args: &Vec<SysResVO>) -> Vec<String> {
        let mut ids = vec![];
        for x in args {
            ids.push(x.id.clone().unwrap_or_default());
            if let Some(childs) = &x.childs{
                let child_ids = self.make_res_ids(childs);
                for child in child_ids {
                    ids.push(child);
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
        let all = CONTEXT.rbatis.fetch_list::<SysRes>().await?;
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
            result.insert(x.id.clone().unwrap_or_default(), x);
        }
        return Ok(result);
    }

    /// 查找res数组
    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRes>> {
        Ok(CONTEXT
            .rbatis
            .fetch_list_by_wrapper(CONTEXT.rbatis.new_wrapper().r#in(SysRes::id(), ids))
            .await?)
    }

    /// 查找res数组
    pub fn finds_res(&self, ids: &Vec<String>, all_res: &BTreeMap<String, SysResVO>) -> Vec<SysResVO> {
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
        let list = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysRes>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .is_null(SysRes::parent_id())
                    .order_by(false, &[SysRes::create_date()]),
            )
            .await?;
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
