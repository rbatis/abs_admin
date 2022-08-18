use crate::domain::table::*;
crud!(SysRoleRes {});
impl_select!(SysRoleRes{select_by_role_id(role_ids: &[String]) =>
    "`where role_id in (`
       trim ',': for _,item in role_ids:
           `#{item},`
      `)`"});
