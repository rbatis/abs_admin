use crate::service::CONTEXT;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{DateType, KeepType, Packer, Rolling, RollingType};
use std::time::Duration;

pub fn init_log() {
    //init fast log
    let mut cfg = Config::new().level(str_to_log_level(&CONTEXT.config.log_level));
    let log_rolling = CONTEXT.config.log_rolling.as_str();
    let rolling_type;
    if log_rolling.ends_with("B") {
        rolling_type = RollingType::BySize(str_to_temp_size(&CONTEXT.config.log_rolling));
    } else if log_rolling.to_lowercase().as_str() == "hour"
        || log_rolling.to_lowercase().as_str() == "minute"
        || log_rolling.to_lowercase().as_str() == "day" {
        match log_rolling.to_lowercase().as_str() {
            "hour" => {
                rolling_type = RollingType::ByDate(DateType::Hour);
            }
            "minute" => {
                rolling_type = RollingType::ByDate(DateType::Minute);
            }
            "day" => {
                rolling_type = RollingType::ByDate(DateType::Day);
            }
            _ => {
                panic!("unknown log_rolling {}",log_rolling);
            }
        }
    } else {
        panic!("unknown log_rolling {}",log_rolling);
    }
    cfg = cfg.file_split(&CONTEXT.config.log_dir,
                         Rolling::new(rolling_type),
                         str_to_keep_type(&CONTEXT.config.log_keep_type),
                         choose_packer(&CONTEXT.config.log_pack_compress),
    );
    if CONTEXT.config.debug {
        cfg = cfg.console();
    }
    cfg = cfg.chan_len(CONTEXT.config.log_chan_len);
    let _ = fast_log::init(cfg);
    if CONTEXT.config.debug == false {
        println!("[abs_admin] release_mode is up! [file_log] open,[console_log] disabled!");
    }
}

fn choose_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        // "lz4" => Box::new(fast_log::plugin::packer::LZ4Packer {}),
        // "zip" => Box::new(fast_log::plugin::packer::ZipPacker {}),
        // "gzip" => Box::new(fast_log::plugin::packer::GZipPacker {}),
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

fn str_to_keep_type(arg: &str) -> KeepType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            KeepType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            KeepType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => KeepType::All,
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
