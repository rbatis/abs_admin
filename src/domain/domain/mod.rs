use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

///权限资源表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
}

///角色资源关系表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleRes {
    pub id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    //资源id
    pub res_id: Option<String>,
    pub create_date: Option<NaiveDateTime>,
}

///后台用户表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<NaiveDateTime>,
}

///用户角色关系表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysUserRole {
    pub id: Option<String>,
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    pub create_date: Option<NaiveDateTime>,
}