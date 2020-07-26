use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::wrapper::Wrapper;
use rbatis_core::db::DriverType;

use crate::dao::RB;
use crate::domain::BizRes;
use crate::bean::dto::ResPageDTO;


/// 资源服务
pub struct ResService {}

impl ResService {
    pub async fn page(&self,arg: &ResPageDTO) -> rbatis_core::Result<Page<BizRes>> {
        let w = Wrapper::new(&RB.driver_type()?);
        let page_req = PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10));
        let data: rbatis_core::Result<Page<BizRes>> = RB.fetch_page_by_wrapper("", &w, &page_req).await;
        data
    }
}