use crate::domain::table::*;
crud!(SysUserRole {});
impl_select!(SysUserRole{select_list_in_user_id(user_ids:&[String])=>
    "`where user_id in (`
     trim ',': for _,v in user_ids:
        `#{v},`
    `)`"});
