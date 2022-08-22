use crate::domain::table::*;
//SysRes
crud!(SysRes {});
impl_select_page!(SysRes{select_page(dto: &crate::domain::dto::ResPageDTO) =>
    "`where del = 0 `
      if dto.name!=null && dto.name!= '':
         ` and name like #{'%'+dto.name+'%'}`
      ` and parent_id IS NULL`
      if !sql.contains('count'):
        ` order by create_date desc`"});
impl_select!(SysRes{select_by_permission_or_name(permission:&str,name:&str) => "`where permission = #{permission} or name = #{name}`"});
impl_select!(SysRes{select_by_ids(ids:&Vec<String>)=>
    "`where id in (`
      trim ',': for _,id in ids:
         #{id},
     `)`"});
impl_select!(SysRes{select_by_parent_id_null()=>"`where parent_id IS NULL order by create_date desc`"});
