use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

///服务启动配置
pub struct BootConfig {
    ///当前服务地址
    pub server_url: String,
    ///日志路径
    pub log_path: String,
    ///redis地址
    pub redis_url: String,
    ///mysql地址
    pub mysql_url: String,
}

///默认配置
impl Default for BootConfig {
    fn default() -> Self {
        let mut f = File::open("src/boot.yml").expect("boot.yml not exist!");
        let mut data = String::new();
        f.read_to_string(&mut data);

        let docs = YamlLoader::load_from_str(&data).unwrap();
        println!("docs: {:?}", docs);

        let server_url= get_cfg(&docs, "server_url").expect("boot.yml/server_url not exist!");
        let log_path= get_cfg(&docs, "log_path").expect("boot.yml/log_path not exist!");
        let redis_url= get_cfg(&docs, "redis_url").expect("boot.yml/redis_url not exist!");
        let mysql_url= get_cfg(&docs, "mysql_url").expect("boot.yml/mysql_url not exist!");

        Self {
            server_url: server_url.as_str().unwrap_or("").to_string(),
            log_path: log_path.as_str().unwrap_or("").to_string(),
            redis_url: redis_url.as_str().unwrap_or("").to_string(),
            mysql_url: mysql_url.as_str().unwrap_or("").to_string(),
        }
    }
}


fn get_cfg<'a>(docs: &'a Vec<Yaml>, key: &str) -> Option<&'a Yaml> {
    for x in docs {
        match x {
            Yaml::Hash(hash) => {
                let v = hash.get(&Yaml::String(key.to_string()));
                if v.is_some() {
                    return v;
                }
            }
            _ => {}
        }
    }
    return Some(&Yaml::Null);
}