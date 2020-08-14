pub mod app_config;

use app_config::ApplicationConfig;

//当前服务配置
lazy_static! {
      pub static ref CONFIG:ApplicationConfig=ApplicationConfig::default();
}