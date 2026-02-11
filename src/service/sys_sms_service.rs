use crate::context::CONTEXT;
use crate::error::{Error, Result};
use crate::error_info;

pub struct SysSmsService {}

impl SysSmsService {
    ///Send verification code
    pub async fn send_verify_sms(&self, account: &str, sms_code: &str) -> Result<()> {
        let _r = CONTEXT
            .cache_service
            .set_string_ex(
                &format!("{},{}", CONTEXT.config.sms_cache_send_key_prefix, account),
                &sms_code,
                Some(std::time::Duration::from_secs(5 * 60)),
            )
            .await?;
        let account = account.to_string();
        let sms_code = sms_code.to_string();
        log::info!("Verifying sms code account:{} = {}", account, sms_code);
        _ = self.do_send_check_sms(account, sms_code).await?;
        Ok(())
    }

    ///Verifying verification code
    pub async fn do_verify_sms(&self, account: &str, sms_code: &str) -> Result<bool> {
        if sms_code.is_empty() && cfg!(debug_assertions) == false {
            return Err(Error::from(error_info!("please_send_code")));
        }
        let sms: Result<String> = CONTEXT
            .cache_service
            .get_string(&format!(
                "{},{}",
                CONTEXT.config.sms_cache_send_key_prefix, account
            ))
            .await;
        match sms {
            Ok(v) => {
                let eq = v.eq(&sms_code);
                Ok(eq)
            }
            _ => Err(Error::from(error_info!("please_send_code"))),
        }
    }

    ///TODO do send sms
    async fn do_send_check_sms(&self, _phone_number: String, _code: String) -> Result<()> {
       log::info!("Sending code to: {}, code: {}", _phone_number, _code);
       Ok(())
    }
}
