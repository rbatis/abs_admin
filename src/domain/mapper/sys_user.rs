use crate::domain::table::*;
crud!(SysUser {});

impl_select_page!(SysUser{select_page(name:&str,account:&str)=>
    "`where del = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    if account != '':
      ` and account like #{'%'+account+'%'}`
    if !sql.contains('count'):
     ` order by create_date desc`"});
