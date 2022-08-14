///服务启动配置
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    pub log_pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    ///日志等级
    pub log_level: String,
    ///短信缓存队列（mem/redis）
    pub sms_cache_send_key_prefix: String,
    ///jwt 秘钥
    pub jwt_secret: String,
    ///白名单接口
    pub white_list_api: Vec<String>,
    ///权限缓存类型
    pub cache_type: String,
    ///重试
    pub login_fail_retry: u64,
    ///重试等待时间
    pub login_fail_retry_wait_sec: u64,
}

///默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../application.yml");
        //读取配置
        let result: ApplicationConfig =
            serde_yaml::from_str(yml_data).expect("load config file fail");
        if result.debug {
            println!("[abs_admin] load config:{:?}", result);
            println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}
