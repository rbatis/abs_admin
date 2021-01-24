use crate::config::CONFIG;
use crate::domain::domain::Sms;
use crate::service::REDIS_SERVICE;
use rbatis::core::{Error, Result};
use std::collections::HashMap;

pub struct SysSmsService {}

impl SysSmsService {
    ///发送验证码
    pub async fn send_verify_sms(&self, account: &str, sms_code: &str) -> Result<()> {
        let mut templete_arg = HashMap::new();
        //短信类型：验证码
        templete_arg.insert("sms_type".to_string(), "verify_sms".to_string());
        //验证码值
        templete_arg.insert("sms_code".to_string(), sms_code.to_string());
        let r = REDIS_SERVICE
            .set_json(
                &(CONFIG.sms_redis_send_key_prefix.to_string() + account),
                &Sms {
                    account: account.to_string(),
                    args: templete_arg,
                },
            )
            .await?;
        return Ok(());
    }

    ///校验验证码
    pub async fn do_verify_sms(&self, account: &str, sms_code: &str) -> Result<bool> {
        let sms: Option<Sms> = REDIS_SERVICE
            .get_json(&(CONFIG.sms_redis_send_key_prefix.to_string() + account))
            .await?;
        match sms {
            Some(v) => {
                let sms_code_cached = v.args.get("sms_code");
                return Ok(sms_code_cached.eq(&Some(&sms_code.to_string())));
            }
            _ => {
                return Err(Error::from("请发送验证码!"));
            }
        }
    }
}
