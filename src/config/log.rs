use crate::config::CONFIG;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;

pub fn init_log() {
    fast_log::init_split_log("target/logs/",
                             1000,
                             LogSize::MB(100),
                             true,
                             RollingType::KeepNum(20),
                             log::Level::Info,
                             None,
                             CONFIG.debug);
}


