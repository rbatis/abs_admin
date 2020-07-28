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
        Self {
            server_url: "127.0.0.1:8000".to_string(),
            log_path: "requests.log".to_string(),
            redis_url: "redis://127.0.0.1:6379".to_string(),
            mysql_url: "mysql://root:123456@localhost:3306/test".to_string(),
        }
    }
}