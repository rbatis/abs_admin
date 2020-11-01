use fast_log::fast_log::LogAppender;
use fast_log::plugin::file::FileAppender;
use fast_log::plugin::console::ConsoleAppender;
use fast_log::filter::ModuleFilter;
use crate::config::CONFIG;

pub fn init_log() {
    //自定义日志追加器
    let mut appenders: Vec<Box<dyn LogAppender>> = vec![
        Box::new(FileAppender::new("requests.log"))
    ];
    if CONFIG.debug {
        appenders.push(Box::new(ConsoleAppender {}));
    }
    //自定义日志过滤
    fast_log::init_custom_log(appenders, 1000, log::Level::Info, Box::new(
        //NoFilter{}//无过滤

        //按模块过滤
        ModuleFilter {
            //只包含
            contains: None,
            //只排除
            exclude_contains: Some(vec![
                "sqlx".to_string()
            ]),
        }
    ));
}


