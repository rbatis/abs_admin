use std::collections::HashMap;
use rbs::to_value;

use crate::error::Error;

/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize,Default)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub server_url: String,
    pub redis_url: String,
    pub db_url: String,
    pub db_pool_len: usize,
    pub db_pool_timeout: usize,
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
    pub datetime_format: String,
    pub errors: HashMap<String, String>,
    pub error_infos: Option<HashMap<String, String>>,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        // let js_data = include_str!("../../application.json5");
        let js_data = std::fs::read_to_string("application.json5").unwrap();
        //load config
        let mut result: ApplicationConfig =
            json5::from_str(&js_data).expect("load config file fail");
        result.init_infos();
        result.debug = cfg!(debug_assertions);
        if result.debug {
            println!("[abs_admin] {}", to_value!(&result));
            println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }

    pub fn get_error_info(&self, code: &str) -> String {
        match self.errors.get(code) {
            None => match self.errors.get("-1") {
                None => "unknown error".to_string() ,
                Some(v) =>v.to_string(),
            },
            Some(v) =>v.to_string() ,
        }
    }

    pub fn get_error(&self, code: &str) -> Error {
        let error_info = self.get_error_info(code);
        Error::CE(code.to_string(), error_info)
    }

    pub fn get_error_arg(&self, code: &str, msg: String) -> Error {
        let error_info = self.get_error_info(code);
        Error::CE(code.to_string(), error_info.replace("{}", &msg))
    }

    pub fn init_infos(&mut self) {
        self.error_infos = Some(HashMap::new());
        for (k, error) in &self.errors {
            let mut error = error.to_string();
            if error.contains(',') {
                error = error[0..error.find(',').unwrap()].to_string();
            }
            self.error_infos
                .as_mut()
                .unwrap()
                .insert(error, k.to_string());
        }
    }
}

#[macro_export]
macro_rules! error_info {
    ($code: expr) => {
        $crate::service::CONTEXT.config.get_error($code)
    };
    ($code: expr, $msg: expr) => {
        $crate::service::CONTEXT.config.get_error_arg($code, $msg)
    };
}
