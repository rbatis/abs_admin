use std::collections::{BTreeMap, HashMap};

use rbatis::crud::{Skip, CRUD};
use rbatis::plugin::page::{Page, PageRequest};

use crate::domain::domain::SysDict;
use crate::domain::dto::{DictEditDTO, DictPageDTO};
use crate::domain::vo::SysDictVO;
use crate::error::Error;
use crate::error::Result;
use crate::service::cache_service::ICacheService;
use crate::service::CONTEXT;
use crate::util::string::IsEmpty;

const DICT_KEY: &'static str = "sys_dict:all";

/// 资源服务
pub struct SysDictService {}

impl SysDictService {
    ///资源分页
    pub async fn page(&self, arg: &DictPageDTO) -> Result<Page<SysDictVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysDict>(
                &CONTEXT
                    .rbatis
                    .new_wrapper()
                    .do_if(!arg.code.is_empty(), |w| w.eq("code", &arg.code))
                    .do_if(!arg.name.is_empty(), |w| w.like("name", &arg.name))
                    .is_null("parent_id")
                    .order_by(false, &["create_date"]),
                &page_req,
            )
            .await?;
        let all_dict = self.finds_all_map().await?;
        let mut all_dict_vo = HashMap::new();
        for (k, v) in all_dict {
            all_dict_vo.insert(k, SysDictVO::from(&v));
        }
        let mut datas = vec![];
        for x in data.records {
            let mut vo = SysDictVO::from(&x);
            vo.set_childs_recursive(&all_dict_vo);
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
    pub async fn add(&self, arg: &SysDict) -> Result<u64> {
        let old: Vec<SysDict> = CONTEXT
            .rbatis
            .fetch_list_by_wrapper(
                &CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq("code", &arg.code)
                    .and()
                    .eq("name", &arg.name),
            )
            .await?;
        if old.len() > 0 {
            return Err(Error::from(format!("字典已存在! {:?}", &arg.name)));
        }
        let result = Ok(CONTEXT.rbatis.save(arg, &[]).await?.rows_affected);
        self.update_cache().await?;
        return result;
    }

    ///修改资源
    pub async fn edit(&self, arg: &DictEditDTO) -> Result<u64> {
        let mut data = SysDict {
            id: arg.id.clone(),
            parent_id: arg.parent_id.clone(),
            name: arg.name.clone(),
            code: arg.code.clone(),
            state: arg.state.clone(),
            create_date: None,
        };
        let result = Ok(CONTEXT
            .rbatis
            .update_by_wrapper(
                &mut data,
                &CONTEXT.rbatis.new_wrapper().eq("id", &arg.id),
                &[Skip::Column("id"), Skip::Column("create_date")],
            )
            .await?);
        self.update_cache().await?;
        return result;
    }

    ///删除资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let vo = self
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::from(format!("用户:{:?} 不存在！", id)))?;
        let vo_list = vec![vo];
        let vo_id_list = self.make_dict_ids(&vo_list);
        let num = CONTEXT
            .rbatis
            .remove_batch_by_column::<SysDict, _>("id", &vo_id_list)
            .await?;

        self.update_cache().await?;

        Ok(num)
    }

    pub fn make_dict_ids(&self, args: &Vec<SysDictVO>) -> Vec<String> {
        let mut ids = vec![];
        for x in args {
            ids.push(x.id.clone().unwrap_or_default());
            match &x.childs {
                Some(childs) => {
                    let child_ids = self.make_dict_ids(childs);
                    for child in child_ids {
                        ids.push(child);
                    }
                }
                _ => {}
            }
        }
        ids
    }

    /// 查找res数组
    pub async fn finds_all(&self) -> Result<Vec<SysDict>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysDict>>>(DICT_KEY)
            .await;
        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        if CONTEXT.config.debug {
            log::info!("[abs_admin] get from redis:{}", DICT_KEY);
        }
        return Ok(js?.unwrap_or_default());
    }

    /// 更新所有
    pub async fn update_cache(&self) -> Result<Vec<SysDict>> {
        let all = CONTEXT.rbatis.fetch_list::<SysDict>().await?;
        CONTEXT.cache_service.set_json(DICT_KEY, &all).await?;
        return Ok(all);
    }

    /// 查找res数组
    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysDict>> {
        let all = self.finds_all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.id.clone().unwrap_or_default(), x);
        }
        return Ok(result);
    }

    /// 查找res数组
    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysDict>> {
        Ok(CONTEXT
            .rbatis
            .fetch_list_by_wrapper(&CONTEXT.rbatis.new_wrapper().r#in("id", ids))
            .await?)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<SysDictVO>> {
        let all_dict = self.finds_all_map().await?;
        let mut all_dict_vo = HashMap::new();
        for (k, v) in all_dict {
            all_dict_vo.insert(k, SysDictVO::from(&v));
        }
        let mut dict_vo = None;
        for (k, v) in &all_dict_vo {
            if k.eq(id) {
                let mut _v = v.clone();
                _v.set_childs_recursive(&all_dict_vo);
                dict_vo = Some(_v);
            }
        }
        Ok(dict_vo)
    }

    /// 查找dict数组
    pub fn finds_dict(
        &self,
        ids: &Vec<String>,
        all_dict: &BTreeMap<String, SysDict>,
    ) -> Vec<SysDict> {
        let mut res = vec![];
        //filter res id
        for (k, v) in all_dict {
            for x in ids {
                if k.eq(x) {
                    res.push(v.clone());
                }
            }
        }
        res
    }

    ///顶层权限
    pub async fn finds_layer_top(&self) -> Result<Vec<SysDictVO>> {
        let list = CONTEXT
            .rbatis
            .fetch_list_by_wrapper::<SysDict>(
                &CONTEXT
                    .rbatis
                    .new_wrapper()
                    .is_null("parent_id")
                    .order_by(false, &["create_date"]),
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
        all_dict: &BTreeMap<String, SysDict>,
    ) -> Result<Vec<SysDictVO>> {
        let res = self.finds_dict(ids, &all_dict);
        //find tops
        let mut tops = vec![];
        for item in res {
            //parent id null, it is an top resource
            if item.parent_id.is_none() {
                tops.push(SysDictVO::from(&item));
            }
        }
        //find child
        for mut item in &mut tops {
            self.loop_find_childs(&mut item, all_dict);
        }
        Ok(tops)
    }

    ///死循环找出父-子 关联关系数组
    pub fn loop_find_childs(&self, arg: &mut SysDictVO, all_dict: &BTreeMap<String, SysDict>) {
        let mut childs: Option<Vec<SysDictVO>> = None;
        for (key, x) in all_dict {
            if x.parent_id.is_some() && x.parent_id.eq(&arg.id) {
                let mut item = SysDictVO::from(x);
                self.loop_find_childs(&mut item, all_dict);
                match &mut childs {
                    Some(childs) => {
                        childs.push(item);
                    }
                    None => {
                        let mut vec = vec![];
                        vec.push(item);
                        childs = Some(vec);
                    }
                }
            }
        }
        if childs.is_some() {
            arg.childs = childs;
        }
    }
}
