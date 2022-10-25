#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]

#[macro_use]
extern crate rbatis;

#[macro_use]
pub mod util;
pub mod config;
pub mod controller;
pub mod domain;
pub mod error;
pub mod middleware;
pub mod service;
