use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::wrapper::Wrapper;
use rbatis_core::db::DriverType;
use rbatis_core::Result;

use crate::dao::RB;
use crate::domain::dto::ResPageDTO;
use crate::domain::domain::SysRes;


/// 资源服务
pub struct SysResService {}

impl SysResService {
    ///资源分页
    pub async fn page(&self,arg: &ResPageDTO) -> Result<Page<SysRes>> {
        let page_req = PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10));
        let data = RB.fetch_page_by_wrapper("", &RB.new_wrapper(), &page_req).await?;
        Ok(data)
    }

    ///添加资源
    pub async fn add(&self, arg:&SysRes) ->Result<u64>{
        RB.save("",arg).await
    }
}