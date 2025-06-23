use crate::context::CONTEXT;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{DateType, KeepType, Packer, Rolling, RollingType};
use std::time::Duration;
use rbatis::rbdc::DateTime;

pub fn init_log() {
    //init fast log
    let mut cfg = Config::new()
        .chan_len(CONTEXT.config.log_chan_len)
        .level(parse_log_level(&CONTEXT.config.log_level))
        .file_split(&CONTEXT.config.log_dir,
                         Rolling::new(parse_rolling_type(CONTEXT.config.log_rolling.as_str())),
                         parse_keep_type(&CONTEXT.config.log_keep_type),
                         parse_packer(&CONTEXT.config.log_pack_compress),
    );
    if CONTEXT.config.debug() {
        cfg = cfg.console();
    }
    let _ = fast_log::init(cfg);
    if CONTEXT.config.debug() == false {
        println!("[abs_admin] release_mode is up! [file_log] open,[console_log] disabled!");
    }
}

fn parse_rolling_type(log_rolling: &str) -> RollingType {
    let lower = log_rolling.to_lowercase();
    let rolling_type;
    if log_rolling.ends_with("B") {
        rolling_type = RollingType::BySize(parse_log_size(&CONTEXT.config.log_rolling));
    } else if lower.as_str().ends_with("minute")
        || lower.as_str().ends_with("hour")
        || lower.as_str().ends_with("day") {
        match lower.as_str() {
            "minute" => {
                rolling_type = RollingType::ByDate(DateType::Minute);
            }
            "hour" => {
                rolling_type = RollingType::ByDate(DateType::Hour);
            }
            "day" => {
                rolling_type = RollingType::ByDate(DateType::Day);
            }
            _ => {
                if lower.ends_with("minute") {
                    let value: u64 = lower.trim_end_matches("minute").parse().expect("parse number fail");
                    rolling_type = RollingType::ByDuration((DateTime::now().0, Duration::from_secs(value * 60)));
                } else if lower.ends_with("hour") {
                    let value: u64 = lower.trim_end_matches("hour").parse().expect("parse number fail");
                    rolling_type = RollingType::ByDuration((DateTime::now().0, Duration::from_secs(value * 60 * 60)));
                } else if lower.ends_with("day") {
                    let value: u64 = lower.trim_end_matches("day").parse().expect("parse number fail");
                    rolling_type = RollingType::ByDuration((DateTime::now().0, Duration::from_secs(value * 24 * 60 * 60)));
                } else {
                    panic!("unknown log_rolling '{}'", log_rolling);
                }
            }
        }
    } else {
        panic!("unknown log_rolling '{}'", log_rolling);
    }
    rolling_type
}

fn parse_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        // "lz4" => Box::new(fast_log::plugin::packer::LZ4Packer {}),
        // "zip" => Box::new(fast_log::plugin::packer::ZipPacker {}),
        // "gzip" => Box::new(fast_log::plugin::packer::GZipPacker {}),
        _ => Box::new(fast_log::plugin::packer::LogPacker {}),
    }
}

fn parse_log_size(arg: &str) -> LogSize {
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

fn parse_keep_type(arg: &str) -> KeepType {
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
        arg if arg.to_uppercase().as_str() == "ALL" => {
            KeepType::All
        }
        _ => {
            panic!("unknown keep_type '{}'", arg)
        }
    }
}

fn parse_log_level(arg: &str) -> log::LevelFilter {
    match arg {
        "off" => log::LevelFilter::Off,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Info,
    }
}
