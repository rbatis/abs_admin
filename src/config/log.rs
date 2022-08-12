use crate::service::CONTEXT;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{FileSplitAppender, Packer, RollingType};
use std::time::Duration;

pub fn init_log() {
    //create log dir
    std::fs::create_dir_all(&CONTEXT.config.log_dir);
    //init fast log
    let mut cfg = Config::new()
        .level(str_to_log_level(&CONTEXT.config.log_level))
        .custom(FileSplitAppender::new(
            &CONTEXT.config.log_dir,
            str_to_temp_size(&CONTEXT.config.log_temp_size),
            str_to_rolling(&CONTEXT.config.log_rolling_type),
            choose_packer(&CONTEXT.config.log_pack_compress),
        ));
    if CONTEXT.config.debug {
        cfg = cfg.console();
    }
    fast_log::init(cfg);
    if CONTEXT.config.debug == false {
        println!("[abs_admin] release_mode is up! [file_log] open,[console_log] disabled!");
    }
}

fn choose_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        #[cfg(feature = "lz4")]
        "lz4" => Box::new(fast_log::plugin::packer::LZ4Packer {}),
        #[cfg(feature = "zip")]
        "zip" => Box::new(fast_log::plugin::packer::ZipPacker {}),
        #[cfg(feature = "gzip")]
        "gzip" => Box::new(fast_log::plugin::packer::GZipPacker {}),
        _ => Box::new(fast_log::plugin::packer::LogPacker {}),
    }
}

fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::LevelFilter {
    return match arg {
        "off" => log::LevelFilter::Off,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
    };
}
