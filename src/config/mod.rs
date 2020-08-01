pub mod boot_config;

use boot_config::BootConfig;

//当前服务配置
lazy_static! {
      pub static ref BOOT_CONFIG:BootConfig=BootConfig::default();
}