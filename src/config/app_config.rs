use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

///服务启动配置
pub struct ApplicationConfig {
    pub debug: bool,
    ///当前服务地址
    pub server_url: String,
    ///日志路径
    pub log_path: String,
    ///redis地址
    pub redis_url: String,
    ///mysql地址
    pub mysql_url: String,

    ///日志目录 "target/logs/"
    pub log_dir: String,
    ///1000
    pub log_cup: i64,
    ///"100MB"
    pub log_temp_size: String,
    ///true
    pub log_zip: bool,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    ///日志等级
    pub log_level: String,
}

///默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {
        let mut yml_data = String::new();
        File::open("src/application.yml")
            .expect("application.yml not exist!")
            .read_to_string(&mut yml_data);

        let docs = YamlLoader::load_from_str(&yml_data).unwrap();
        //读取配置
        Self {
            debug: get_cfg(&docs, "debug").as_bool().unwrap_or(true),
            server_url: get_cfg(&docs, "server_url")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            log_path: get_cfg(&docs, "log_path").as_str().unwrap_or("").to_owned(),
            redis_url: get_cfg(&docs, "redis_url")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            mysql_url: get_cfg(&docs, "mysql_url")
                .as_str()
                .unwrap_or("")
                .to_owned(),

            log_dir: get_cfg(&docs, "log_dir")
                .as_str()
                .unwrap_or("target/logs/")
                .to_owned(),
            log_cup: get_cfg(&docs, "log_cup")
                .as_i64()
                .unwrap_or(10000)
                .to_owned(),
            log_temp_size: get_cfg(&docs, "log_temp_size")
                .as_str()
                .unwrap_or("100MB")
                .to_owned(),
            log_zip: get_cfg(&docs, "log_zip").as_bool().unwrap_or(false),
            log_rolling_type: get_cfg(&docs, "log_rolling_type")
                .as_str()
                .unwrap_or("All")
                .to_owned(),
            log_level: get_cfg(&docs, "log_level")
                .as_str()
                .unwrap_or("info")
                .to_owned(),
        }
    }
}

/// 获取配置
/// key: 需要获取配置的key
fn get_cfg<'a>(docs: &'a Vec<Yaml>, key: &str) -> &'a Yaml {
    for x in docs {
        match x {
            Yaml::Hash(hash) => {
                let v = hash.get(&Yaml::String(key.to_string()));
                if v.is_some() {
                    return v.unwrap();
                }
            }
            _ => {}
        }
    }
    panic!(format!("in application.yml key: '{}' not exist!", key))
}
