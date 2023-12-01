use std::collections::HashMap;

/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub server_url: String,
    pub redis_url: String,
    pub database_url: String,
    pub log_dir: String,
    pub log_temp_size: String,
    pub log_pack_compress: String,
    pub log_rolling_type: String,
    pub log_level: String,
    pub log_type: String,
    pub log_chan_len: Option<usize>,
    pub sms_cache_send_key_prefix: String,
    pub jwt_secret: String,
    pub jwt_exp: usize,
    pub jwt_refresh_token: usize,
    pub white_list_api: Vec<String>,
    pub cache_type: String,
    pub login_fail_retry: u64,
    pub login_fail_retry_wait_sec: u64,
    pub trash_recycle_days: u64,
    pub errors: HashMap<String, String>,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let js_data = include_str!("../../application.json5");
        //load config
        let mut result: ApplicationConfig = json5::from_str(js_data).expect("load config file fail");
        if cfg!(debug_assertions) {
            result.debug = true;
        } else {
            result.debug = false;
        }
        if result.debug {
            println!("[abs_admin] load config:{:?}", result);
            println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}

impl ApplicationConfig {
    pub fn get_error_info(&self, code: &str) -> String {
        match self.errors.get(code) {
            None => {
                match self.errors.get("-1"){
                    None => {
                        "unknown error".to_string()
                    }
                    Some(v) => {
                        v.to_string()
                    }
                }
            }
            Some(v) => {
                v.as_str().to_string()
            }
        }
    }
}

#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::service::CONTEXT.config.get_error_info($code)
    };
}