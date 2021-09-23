use crate::domain::dto::CatpchaDTO;
use crate::domain::vo::RespVO;
use crate::service::{ICacheService, CONTEXT};
use actix_web::{web, HttpResponse, Responder};
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;
use crate::util::string::IsEmpty;
use crate::error::Error;

///图形验证码接口(注意，debug模式无论缓存是否连接成功都返回图片，release模式则校验缓存(例如redis)是否存储成功)
/// 请求方式 GET
/// 例子：
/// http://localhost:8000/admin/captcha?account=18900000000
///
pub async fn captcha(arg: web::Query<CatpchaDTO>) -> impl Responder {
    if arg.account.is_empty() {
        return RespVO::<()>::from_error("-1", &Error::from("account is empty!")).resp_json();
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
        log::info!("account:{},captcha:{}",arg.account.as_ref().unwrap(),&captcha_str);
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
                return RespVO::from_result(&result).resp_json();
            }
        }
    }
    HttpResponse::Ok()
        .set_header("Access-Control-Allow-Origin", "*")
        .set_header("Cache-Control", "no-cache")
        .content_type("image/png")
        .body(png)
}
