pub mod application;

use application::ApplicationConfig;

//当前服务配置
lazy_static! {
      pub static ref CONFIG:ApplicationConfig=ApplicationConfig::default();
}