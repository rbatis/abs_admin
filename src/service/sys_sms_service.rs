use crate::domain::domain::Sms;
use crate::error::{Error, Result};
use crate::service::cache_service::ICacheService;
use crate::service::CONTEXT;
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
        let r = CONTEXT
            .cache_service
            .set_json(
                &format!("{},{}", CONTEXT.config.sms_cache_send_key_prefix, account),
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
        let sms: Option<Sms> = CONTEXT
            .cache_service
            .get_json(&format!(
                "{},{}",
                CONTEXT.config.sms_cache_send_key_prefix, account
            ))
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
