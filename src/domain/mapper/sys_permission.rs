use crate::domain::table::*;
//SysPermission
crud!(SysPermission {});
impl_select_page!(SysPermission{select_page(dto: &crate::domain::dto::ResPageDTO) =>
    "`where 0 = 0 `
      if dto.name!=null && dto.name!= '':
         ` and name like #{'%'+dto.name+'%'}`
      ` and parent_id IS NULL`
      if !sql.contains('count'):
        ` order by create_date desc`"});
impl_select!(SysPermission{select_by_permission_or_name(permission:&str,name:&str) => "`where permission = #{permission} or name = #{name}`"});
impl_select!(SysPermission{select_by_parent_id_null()=>"`where parent_id IS NULL order by create_date desc`"});
