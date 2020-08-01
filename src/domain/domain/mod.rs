
use rbatis::crud::CRUDEnable;
use serde::{Deserialize, Serialize};

///权限资源表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRes {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<String>,
}

impl CRUDEnable for SysRes {
    type IdType = String;
}

///角色表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<String>,
}

impl CRUDEnable for SysRole {
    type IdType = String;
}

///角色资源关系表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleRes {
    pub id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    //资源id
    pub res_id: Option<String>,
    pub create_time: Option<String>,
}

impl CRUDEnable for SysRoleRes {
    type IdType = String;
}

///后台用户表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<String>,
}

impl CRUDEnable for SysUser {
    type IdType = String;
}

///用户角色关系表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysUserRole {
    pub id: Option<String>,
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    pub create_time: Option<String>,
}

impl CRUDEnable for SysUserRole {
    type IdType = String;
}