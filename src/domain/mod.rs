///DDD分层架构，分为
///
/// * 领域层（domain）,该层存放数据库结构体模型
pub mod domain;
/// * 数据传输层（dto，Data Transfer Object ）,存放接口传输的结构体
pub mod dto;
/// * 展示层（vo，View Object），存放展示的结构体
pub mod vo;


pub trait AsStr {
    fn as_str_default(&self) -> &str;
}

impl AsStr for Option<String> {
    fn as_str_default(&self) -> &str {
        match self {
            None => { "" }
            Some(v) => { v.as_str() }
        }
    }
}