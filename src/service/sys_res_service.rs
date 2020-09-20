use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis_core::Result;

use crate::dao::RB;
use crate::domain::domain::SysRes;
use crate::domain::dto::{ResEditDTO, ResPageDTO};
use crate::domain::vo::SysResVO;

/// 资源服务
pub struct SysResService {}

impl SysResService {
    ///资源分页
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysRes>> {
        let page_req = PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10));
        let data = RB.fetch_page_by_wrapper("", &RB.new_wrapper(), &page_req).await?;
        Ok(data)
    }

    ///添加资源
    pub async fn add(&self, arg: &SysRes) -> Result<u64> {
        RB.save("", arg).await
    }

    ///添加资源
    pub async fn edit(&self, arg: &ResEditDTO) -> Result<u64> {
        unimplemented!()
    }

    ///添加资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        unimplemented!()
    }

    /// 查找res数组
    pub async fn finds_all(&self) -> Result<Vec<SysRes>> {
        RB.list("").await
    }

    /// 查找res数组
    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRes>> {
        RB.list_by_wrapper("", RB.new_wrapper().in_array("id", ids)).await
    }

    ///带有层级结构的 res数组
    pub async fn finds_layer(&self, ids: &Vec<String>, all_res: &Vec<SysRes>) -> Result<Vec<SysResVO>> {
        let res = self.finds(&ids).await?;
        //find tops
        let mut tops = vec![];
        for item in res {
            //parent id null, it is an top resource
            if item.parent_id.is_none() {
                tops.push(SysResVO::from(&item));
            }
        }
        //find child
        for mut item in &mut tops {
            self.loop_find_childs(&mut item, all_res);
        }
        Ok(tops)
    }

    ///死循环找出父-子 关联关系数组
    pub fn loop_find_childs(&self, arg: &mut SysResVO, all_res: &Vec<SysRes>) {
        let mut childs = Option::<Vec<SysResVO>>::None;
        for x in all_res {
            if x.parent_id.eq(&x.id) {
                let mut item = SysResVO::from(&x.clone());
                self.loop_find_childs(&mut item, all_res);
                if childs.is_none() {
                    childs = Some(vec![]);
                }
                childs.as_mut().unwrap().push(item);
            }
        }
        arg.childs = childs;
    }
}