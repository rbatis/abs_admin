use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

///服务启动配置
pub struct ApplicationConfig {
    pub debug: bool,
    ///当前服务地址
    pub server_url: String,
    ///redis地址
    pub redis_url: String,
    /// 数据库地址
    pub database_url: String,

    /// 逻辑删除字段
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,

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

    ///短信redis队列
    pub sms_redis_send_key_prefix: String,

    ///jwt 秘钥
    pub jwt_secret: String,

    ///白名单接口
    pub white_list_api: Vec<String>,
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
        let result = Self {
            debug: get_cfg(&docs, "debug").as_bool().unwrap_or(true),
            server_url: get_cfg(&docs, "server_url")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            redis_url: get_cfg(&docs, "redis_url")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            database_url: get_cfg(&docs, "database_url")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            logic_column: get_cfg(&docs, "logic_column")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            logic_un_deleted: get_cfg(&docs, "logic_un_deleted")
                .as_i64()
                .unwrap_or_default(),
            logic_deleted: get_cfg(&docs, "logic_deleted")
            .as_i64()
            .unwrap_or_default(),
            log_dir: get_cfg(&docs, "log_dir")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            log_cup: get_cfg(&docs, "log_cup")
                .as_i64()
                .unwrap_or(0)
                .to_owned(),
            log_temp_size: get_cfg(&docs, "log_temp_size")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            log_zip: get_cfg(&docs, "log_zip").as_bool().unwrap_or(false),
            log_rolling_type: get_cfg(&docs, "log_rolling_type")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            log_level: get_cfg(&docs, "log_level")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            sms_redis_send_key_prefix: get_cfg(&docs, "sms_redis_send_key_prefix")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            jwt_secret: get_cfg(&docs, "jwt_secret")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            white_list_api: to_vec_string(get_cfg(&docs, "white_list_api").as_vec().unwrap().to_vec())
        };

        if result.debug {
            println!("[abs_admin] debug_mode is enable!")
        } else {
            println!("[abs_admin] release_mode is enable!")
        }

        result
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

fn to_vec_string(arg:Vec<Yaml>) -> Vec<String> {
    let mut arr=vec![];
    for x in arg {
        arr.push(x.as_str().unwrap_or("").to_string());
    }
    return arr;
}
