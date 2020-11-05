use fast_log::fast_log::LogAppender;
use fast_log::plugin::file::FileAppender;
use fast_log::plugin::console::ConsoleAppender;
use fast_log::filter::ModuleFilter;
use crate::config::CONFIG;
use fast_log::plugin::file_split::FileSplitAppender;

pub fn init_log() {
    //自定义日志追加器
    let mut appenders: Vec<Box<dyn LogAppender>> = vec![
        Box::new(FileSplitAppender::new("target/log/",1000000,true))
    ];
    if CONFIG.debug {
        appenders.push(Box::new(ConsoleAppender {}));
    }
    //自定义日志过滤
    fast_log::init_custom_log(appenders,
                              1000,
                              log::Level::Info,
                              Box::new(ModuleFilter::new_exclude(vec!["sqlx".to_string()]))
    );
}


