use crate::config::CONFIG;
use crate::domain::domain::Sms;
use crate::service::REDIS_SERVICE;
use rbatis::core::Result;
use std::collections::HashMap;

pub struct SysSmsService {}

impl SysSmsService {
    pub async fn send(&self, account: &str, sms_code: &str) -> Result<()> {
        let mut templete_arg = HashMap::new();
        //短信类型：验证码
        templete_arg.insert("sms_type".to_string(), "verify_code".to_string());
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
}
