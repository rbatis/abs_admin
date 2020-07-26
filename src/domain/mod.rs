use rbatis::crud::CRUDEnable;
use serde::{Deserialize, Serialize};

///权限资源表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BizRes {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<String>,
}

impl CRUDEnable for BizRes {
    type IdType = String;
}

///角色表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BizRole {
    pub id: Option<String>,
    pub name: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<String>,
}

impl CRUDEnable for BizRole {
    type IdType = String;
}

///角色资源关系表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BizRoleRes {
    pub id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    //资源id
    pub res_id: Option<String>,
    pub create_time: Option<String>,
}

impl CRUDEnable for BizRoleRes {
    type IdType = String;
}

///后台用户表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BizAdminUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub del: Option<i32>,
    pub create_time: Option<String>,
}

impl CRUDEnable for BizAdminUser {
    type IdType = String;
}

///用户角色关系表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BizAdminUserRole {
    pub id: Option<String>,
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    pub create_time: Option<String>,
}

impl CRUDEnable for BizAdminUserRole {
    type IdType = String;
}