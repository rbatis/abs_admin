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

/// 字典服务
pub struct SysDictService {}

impl SysDictService {
    ///字典分页
    pub async fn page(&self, arg: &DictPageDTO) -> Result<Page<SysDict>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysDict>(
                CONTEXT
                    .rbatis
                    .new_wrapper()
                    .do_if(!arg.code.is_empty(), |w| w.eq("code", &arg.code))
                    .do_if(!arg.name.is_empty(), |w| w.like("name", &arg.name))
                    .order_by(false, &["create_date"]),
                &page_req,
            )
            .await?;
        Ok(data)
    }

    ///添加字典
    pub async fn add(&self, arg: &SysDict) -> Result<u64> {
        let old: Vec<SysDict> = CONTEXT
            .rbatis
            .fetch_list_by_wrapper(
                CONTEXT
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

    ///修改字典
    pub async fn edit(&self, arg: &DictEditDTO) -> Result<u64> {
        let mut data = SysDict {
            id: arg.id.clone(),
            name: arg.name.clone(),
            code: arg.code.clone(),
            state: arg.state.clone(),
            create_date: None,
        };
        let result = Ok(CONTEXT
            .rbatis
            .update_by_wrapper(
                &mut data,
                CONTEXT.rbatis.new_wrapper().eq("id", &arg.id),
                &[Skip::Column("id"), Skip::Column("create_date")],
            )
            .await?);
        self.update_cache().await?;
        return result;
    }

    ///删除字典
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let num = CONTEXT
            .rbatis
            .remove_batch_by_column::<SysDict, _>("id", &[id])
            .await?;
        if num > 0{
            self.update_cache().await?;
        }
        Ok(num)
    }

    /// 查找字典数组
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

    /// 查找字典数组
    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysDict>> {
        let all = self.finds_all().await?;
        let mut result = BTreeMap::new();
        for x in all {
            result.insert(x.id.clone().unwrap_or_default(), x);
        }
        return Ok(result);
    }

    /// 查找字典数组
    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysDict>> {
        Ok(CONTEXT
            .rbatis
            .fetch_list_by_wrapper(CONTEXT.rbatis.new_wrapper().r#in("id", ids))
            .await?)
    }

    ///根据id查找，id=key
    pub async fn find_by_id(&self, id: &str) -> Result<Option<SysDict>> {
        let v=CONTEXT.rbatis.fetch_by_column::<Option<SysDict>,_>("id",&id.to_owned()).await?;
        Ok(v)
    }
}
