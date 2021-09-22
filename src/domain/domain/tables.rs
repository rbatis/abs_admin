use crate::domain::domain::LoginCheck;
use chrono::NaiveDateTime;

///权限资源表
#[crud_table]
#[derive(Clone, Debug)]
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
    pub create_date: Option<NaiveDateTime>,
}

///角色表
#[crud_table]
#[derive(Clone, Debug)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
}

///角色资源关系表(关系表不使用逻辑删除)
#[crud_table]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SysRoleRes {
    pub id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    //资源id
    pub res_id: Option<String>,
    pub create_date: Option<NaiveDateTime>,
}

///后台用户表
#[crud_table]
#[derive(Clone, Debug)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
}

///用户角色关系表(关系表不使用逻辑删除)
#[crud_table]
#[derive(Clone, Debug)]
pub struct SysUserRole {
    pub id: Option<String>,
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    pub create_date: Option<NaiveDateTime>,
}
///字典表
#[crud_table]
#[derive(Clone, Debug)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
}
