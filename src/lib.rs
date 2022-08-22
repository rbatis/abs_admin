#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]

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
///领域模型模块
pub mod domain;
///错误结构体
pub mod error;
///actix-web中间件
pub mod middleware;
///服务模块
pub mod service;
