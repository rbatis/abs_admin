#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

///工具类
#[macro_use]
pub mod util;
///配置模块
pub mod config;
///接口模块
pub mod controller;
///数据库模块
pub mod dao;
///领域模型模块
pub mod domain;
///错误结构体
pub mod error;
///actix-web中间件
pub mod middleware;
///服务模块
pub mod service;
