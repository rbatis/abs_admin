
use rbatis::crud::CRUDEnable;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

///权限资源表
#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct SysRes {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}

///角色表
#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}

///角色资源关系表
#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleRes {
    pub id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    //资源id
    pub res_id: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}

///后台用户表
#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}

///用户角色关系表
#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct SysUserRole {
    pub id: Option<String>,
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}