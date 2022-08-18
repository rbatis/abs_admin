use crate::domain::table::LoginCheck;
use rbatis::rbdc::datetime::FastDateTime;
///权限资源表
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRes {
    pub id: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //权限
    pub permission: Option<String>,
    //前端-菜单路径
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

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

///角色表
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

crud!(SysRole {});

impl_select!(SysRole{select_list_by_ids(ids:&[String])=>
    "`where id in (`
     trim ',': for _,item in ids:
         #{item},
     )"});

impl_select_page!(SysRole{select_page_by_name(name:&str)=>
    "`where del = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    ` and parent_id IS NULL `
    if !sql.contains('count'):
     `order by create_date desc`"});

///角色资源关系表(关系表不使用逻辑删除)

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRoleRes {
    pub id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    //资源id
    pub res_id: Option<String>,
    pub create_date: Option<FastDateTime>,
}

crud!(SysRoleRes {});
impl_select!(SysRoleRes{select_by_role_id(role_ids: &[String]) =>
    "`where role_id in (`
       trim ',': for _,item in role_ids:
           `#{item},`
      `)`"});

///后台用户表

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

crud!(SysUser {});

impl_select_page!(SysUser{select_page(name:&str,account:&str)=>
    "`where del = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    if account != '':
      ` and account like #{'%'+account+'%'}`
    if !sql.contains('count'):
     ` order by create_date desc`"});

///用户角色关系表(关系表不使用逻辑删除)

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserRole {
    pub id: Option<String>,
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    pub create_date: Option<FastDateTime>,
}

crud!(SysUserRole {});
impl_select!(SysUserRole{select_list_in_user_id(user_ids:&[String])=>
    "`where user_id in (`
     trim ',': for _,v in user_ids:
        `#{v},`
    `)`"});

///字典表

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

crud!(SysDict {});
impl_select_page!(SysDict{select_page(dto: &crate::domain::dto::DictPageDTO) =>
    "`where id!=''`
      if dto.code!=null:
         ` and code = #{dto.code}`
      if dto.name!=null:
         ` and name = #{dto.name}`
      if !sql.contains('count'):
         ` order by create_date `"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_date: Option<FastDateTime>,
}

crud!(SysTrash {});
