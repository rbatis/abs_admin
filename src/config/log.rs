use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use std::time::Duration;
use crate::service::CONTEXT;

pub fn init_log() {
    fast_log::init_split_log(
        &CONTEXT.config.log_dir,
        CONTEXT.config.log_cup as usize,
        str_to_temp_size(&CONTEXT.config.log_temp_size),
        CONTEXT.config.log_zip,
        str_to_rolling(&CONTEXT.config.log_rolling_type),
        str_to_log_level(&CONTEXT.config.log_level),
        None,
        CONTEXT.config.debug,
    );
    if CONTEXT.config.debug == false {
        println!("[abs_admin] release_mode is up! [file_log] open,[console_log] disabled!");
    }
}

fn str_to_temp_size(arg: &str) -> LogSize {
    if arg.ends_with("MB") {
        let end = arg.find("MB").unwrap();
        let num = arg[0..end].to_string();
        return LogSize::MB(num.parse::<usize>().unwrap());
    } else if arg.ends_with("KB") {
        let end = arg.find("KB").unwrap();
        let num = arg[0..end].to_string();
        return LogSize::KB(num.parse::<usize>().unwrap());
    } else if arg.ends_with("GB") {
        let end = arg.find("GB").unwrap();
        let num = arg[0..end].to_string();
        return LogSize::GB(num.parse::<usize>().unwrap());
    } else {
        //default
        return LogSize::MB(100);
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    if arg.starts_with("KeepNum(") {
        let end = arg.find(")").unwrap();
        let num = arg["KeepNum(".len()..end].to_string();
        return RollingType::KeepNum(num.parse::<i64>().unwrap());
    } else if arg.starts_with("KeepTime(") {
        let end = arg.find(")").unwrap();
        let num = arg["KeepTime(".len()..end].to_string();
        return RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()));
    } else {
        //default
        return RollingType::All;
    }
}

fn str_to_log_level(arg: &str) -> log::Level {
    return match arg {
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        "trace" => log::Level::Trace,
        "info" => log::Level::Info,
        "debug" => log::Level::Debug,
        _ => log::Level::Info,
    };
}
