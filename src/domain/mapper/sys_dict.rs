use crate::domain::table::*;
crud!(SysDict {});
impl_select_page!(SysDict{select_page(dto: &crate::domain::dto::DictPageDTO) =>
    "`where id!=''`
      if dto.code!=null:
         ` and code = #{dto.code}`
      if dto.name!=null:
         ` and name = #{dto.name}`
      if !sql.contains('count'):
         ` order by create_date `"});
