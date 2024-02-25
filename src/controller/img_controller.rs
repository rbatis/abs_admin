use crate::domain::dto::CatpchaDTO;
use crate::domain::vo::RespVO;
use crate::error_info;
use crate::service::CONTEXT;
use crate::util::string::IsEmptyString;
use actix_web::{web, HttpResponse, Responder};
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;

/// Image Code interface
/// Http Method GET
/// example：
/// http://localhost:8000/admin/captcha?account=18900000000
///
pub async fn captcha(arg: web::Query<CatpchaDTO>) -> impl Responder {
    if arg.account.is_empty() {
        return RespVO::<()>::from_error(error_info!("account_empty"))
        .resp_json();
    }
    let mut captcha = Captcha::new();
    captcha
        .add_chars(4)
        .apply_filter(Noise::new(0.1))
        .apply_filter(Wave::new(1.0, 10.0).horizontal())
        // .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(160, 60)
        .apply_filter(Dots::new(4));
    let png = captcha.as_png().unwrap();
    let captcha_str = captcha.chars_as_string().to_lowercase();
    if CONTEXT.config.debug {
        log::info!(
            "account:{},captcha:{}",
            arg.account.as_ref().unwrap(),
            &captcha_str
        );
    }
    if arg.account.is_some() {
        let result = CONTEXT
            .cache_service
            .set_string(
                &format!("captch:account_{}", &arg.account.as_ref().unwrap()),
                captcha_str.as_str(),
            )
            .await;
        //println!("{:?}", result);
        if CONTEXT.config.debug == false {
            //release mode, return the error
            if result.is_err() {
                return RespVO::from_result(result).resp_json();
            }
        }
    }
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .content_type("image/png")
        .body(png)
        .into()
}
