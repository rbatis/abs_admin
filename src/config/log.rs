use crate::service::CONTEXT;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_mmap::MmapFile;
use fast_log::plugin::file_split::{FileSplitAppender, Packer, RawFile, RollingType};
use fast_log::FastLogFormat;
use std::time::Duration;

pub fn init_log() {
    //init fast log
    let mut cfg = Config::new().level(str_to_log_level(&CONTEXT.config.log_level))
        .format(FastLogFormat::new().set_display_line_level(log::LevelFilter::Warn));
    match CONTEXT.config.log_type.as_str() {
        "mmap" => {
            cfg = cfg.custom(
                FileSplitAppender::<MmapFile>::new(
                    &CONTEXT.config.log_dir,
                    str_to_temp_size(&CONTEXT.config.log_temp_size),
                    str_to_rolling(&CONTEXT.config.log_rolling_type),
                    choose_packer(&CONTEXT.config.log_pack_compress),
                )
                .unwrap(),
            );
        }
        _ => {
            cfg = cfg.custom(
                FileSplitAppender::<RawFile>::new(
                    &CONTEXT.config.log_dir,
                    str_to_temp_size(&CONTEXT.config.log_temp_size),
                    str_to_rolling(&CONTEXT.config.log_rolling_type),
                    choose_packer(&CONTEXT.config.log_pack_compress),
                )
                .unwrap(),
            );
        }
    }
    if CONTEXT.config.debug {
        cfg = cfg.console();
    }
    cfg = cfg.chan_len(CONTEXT.config.log_chan_len);
    let _ = fast_log::init(cfg);
    if !CONTEXT.config.debug {
        println!("[abs_admin] release_mode is up! [file_log] open,[console_log] disabled!");
    }
}

fn choose_packer(_packer: &str) -> Box<dyn Packer> {
    Box::new(fast_log::plugin::packer::LogPacker {})
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
            let end = arg.find(')').unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(')').unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::LevelFilter {
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
