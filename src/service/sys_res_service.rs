use std::collections::HashMap;

use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::Error;
use rbatis::plugin::page::{Page, PageRequest};

use crate::domain::domain::SysRes;
use crate::domain::dto::{ResEditDTO, ResPageDTO};
use crate::domain::vo::SysResVO;
use crate::service::CONTEXT;

/// 资源服务
pub struct SysResService {}

impl SysResService {
    ///资源分页
    pub async fn page(&self, arg: &ResPageDTO) -> Result<Page<SysResVO>> {
        let page_req = PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let data = CONTEXT
            .rbatis
            .fetch_page_by_wrapper::<SysRes>("", &CONTEXT.rbatis.new_wrapper()
                .is_null("parent_id")
                .order_by(false, &["create_date"]), &page_req)
            .await?;
        let all_res = self.finds_all_map().await?;
        let mut datas = vec![];
        for x in data.records {
            let mut vo = SysResVO::from(&x);
            self.loop_find_childs(&mut vo, &all_res);
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

    ///详情(附带层级数据)
    pub async fn detail(&self, id: &String) -> Result<SysResVO> {
        let res = CONTEXT.rbatis.fetch_by_id::<SysRes>("", id).await?;
        let mut vo = SysResVO::from(&res);
        let all_res = self.finds_all_map().await?;
        self.loop_find_childs(&mut vo, &all_res);
        return Ok(vo);
    }

    ///添加资源
    pub async fn add(&self, arg: &SysRes) -> Result<u64> {
        let old: Option<SysRes> = CONTEXT
            .rbatis
            .fetch_by_wrapper(
                "",
                &CONTEXT
                    .rbatis
                    .new_wrapper()
                    .eq("permission", &arg.permission),
            )
            .await?;
        if old.is_some() {
            return Err(Error::from("权限已存在!"));
        }
        Ok(CONTEXT.rbatis.save("", arg).await?.rows_affected)
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
        CONTEXT.rbatis.update_by_id("", &mut data).await
    }

    ///删除资源
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let num = CONTEXT
            .rbatis
            .remove_by_id::<SysRes>("", &id.to_string())
            .await?;
        //删除父级为id的记录
        CONTEXT
            .rbatis
            .remove_by_wrapper::<SysRes>("", &CONTEXT.rbatis.new_wrapper().eq("parent_id", id))
            .await;
        //删除关联数据
        CONTEXT.sys_role_res_service.remove_by_res_id(id).await;
        return Ok(num);
    }

    /// 查找res数组
    pub async fn finds_all(&self) -> Result<Vec<SysRes>> {
        //TODO 查找的全部数据缓存于Redis，同时 remove，edit方法调用时刷新redis缓存
        CONTEXT.rbatis.fetch_list("").await
    }

    /// 查找res数组
    pub async fn finds_all_map(&self) -> Result<HashMap<String, SysRes>> {
        let all = self.finds_all().await?;
        let mut result = HashMap::new();
        for mut x in all {
            result.insert(x.id.take().unwrap_or_default(), x);
        }
        return Ok(result);
    }

    /// 查找res数组
    pub async fn finds(&self, ids: &Vec<String>) -> Result<Vec<SysRes>> {
        CONTEXT
            .rbatis
            .fetch_list_by_wrapper("", &CONTEXT.rbatis.new_wrapper().r#in("id", ids))
            .await
    }

    /// 查找res数组
    pub fn finds_res(&self, ids: &Vec<String>, all_res: &HashMap<String, SysRes>) -> Vec<SysRes> {
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

    ///带有层级结构的 res数组
    pub async fn finds_layer(
        &self,
        ids: &Vec<String>,
        all_res: &HashMap<String, SysRes>,
    ) -> Result<Vec<SysResVO>> {
        let res = self.finds_res(ids, &all_res);
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
    pub fn loop_find_childs(&self, arg: &mut SysResVO, all_res: &HashMap<String, SysRes>) {
        let mut childs = None;
        for (key, x) in all_res {
            if x.parent_id.is_some() && x.parent_id.eq(&arg.id) {
                let mut item = SysResVO::from(x);
                self.loop_find_childs(&mut item, all_res);
                if childs.is_none() {
                    childs = Some(vec![]);
                }
                childs.as_mut().unwrap().push(item);
            }
        }
        if childs.is_some() && childs.as_ref().unwrap().len() != 0 {
            arg.childs = childs;
        }
    }
}
