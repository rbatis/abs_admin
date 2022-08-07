use rbatis::sql::{IPage, IPageRequest, Page, PageRequest};
use crate::domain::domain::SysDict;
use crate::domain::dto::{DictEditDTO, DictPageDTO};
use crate::domain::vo::SysDictVO;
use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use crate::util::string::IsEmptyString;

const DICT_KEY: &'static str = "sys_dict:all";

/// 字典服务
pub struct SysDictService {}

impl SysDictService {
    ///字典分页
    pub async fn page(&self, arg: &DictPageDTO) -> Result<Page<SysDictVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = SysDict::sys_dict_page(&mut CONTEXT.rbatis.clone(),&PageRequest::from(arg),arg).await?;
        let mut page = Page::<SysDictVO>::from(data);
        Ok(page)
    }

    ///添加字典
    pub async fn add(&self, arg: &SysDict) -> Result<u64> {
        // let old: Vec<SysDict> = CONTEXT
        //     .rbatis
        //     .fetch_list_by_wrapper(
        //         CONTEXT
        //             .rbatis
        //             .new_wrapper()
        //             .eq(SysDict::code(), &arg.code)
        //             .and()
        //             .eq(SysDict::name(), &arg.name),
        //     )
        //     .await?;
        // if old.len() > 0 {
        //     return Err(Error::from(format!("字典已存在! {:?}", &arg.name)));
        // }
        // let result = Ok(CONTEXT.rbatis.save(arg, &[]).await?.rows_affected);
        // self.update_cache().await?;
        // return result;
        todo!()
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
        // let result = Ok(CONTEXT
        //     .rbatis
        //     .update_by_wrapper(
        //         &mut data,
        //         CONTEXT.rbatis.new_wrapper().eq(SysDict::id(), &arg.id),
        //         &[Skip::Column(SysDict::id()), Skip::Column(SysDict::create_date())],
        //     )
        //     .await?);
        // self.update_cache().await?;
        // return result;
        todo!()
    }

    ///删除字典
    pub async fn remove(&self, id: &str) -> Result<u64> {
        //let num = CONTEXT
        //     .rbatis
        //     .remove_batch_by_column::<SysDict, _>(SysDict::id(), &[id])
        //     .await?;
        // if num > 0 {
        //     self.update_cache().await?;
        // }
        // Ok(num)
        todo!()
    }

    /// 更新所有
    pub async fn update_cache(&self) -> Result<()> {
        // let all = CONTEXT.rbatis.fetch_list::<SysDict>().await?;
        // CONTEXT.cache_service.set_json(DICT_KEY, &all).await?;
        // return Ok(());
        todo!()
    }
}
