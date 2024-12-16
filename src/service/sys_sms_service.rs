use crate::error::{Error, Result};
use crate::error_info;
use crate::context::CONTEXT;
use std::collections::HashMap;
use crate::domain::table::sms::Sms;

pub struct SysSmsService {}

impl SysSmsService {
    ///Send verification code
    pub async fn send_verify_sms(&self, account: &str, sms_code: &str) -> Result<()> {
        let mut templete_arg = HashMap::new();
        templete_arg.insert("sms_type".to_string(), "verify_sms".to_string());
        templete_arg.insert("sms_code".to_string(), sms_code.to_string());
        let _r = CONTEXT
            .cache_service
            .set_json(
                &format!("{},{}", CONTEXT.config.sms_cache_send_key_prefix, account),
                &Sms {
                    account: account.to_string(),
                    args: templete_arg,
                },
            )
            .await?;
        Ok(())
    }

    ///Verifying verification code
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
                Ok(sms_code_cached.eq(&Some(&sms_code.to_string())))
            }
            _ => Err(Error::from(error_info!("please_send_code"))),
        }
    }
}
