use crate::domain::dto::CatpchaDTO;
use crate::domain::vo::RespVO;
use crate::service::{CONTEXT, ICacheService};
use actix_web::{web, HttpResponse, Responder};
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;

///图形验证码接口(注意，debug模式无论redis是否连接成功都返回图片，release模式则校验redis是否存储成功)
/// 请求方式 GET
/// 例子：
/// http://localhost:8000/captcha?account=18900000000
///
pub async fn captcha(arg: web::Query<CatpchaDTO>) -> impl Responder {
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
    println!(
        "account:{},captcha:{}",
        arg.account.as_ref().unwrap_or(&"".to_string()),
        &captcha_str
    );
    if arg.account.is_some() {
        let result = CONTEXT
            .cache_service
            .set_string(
                &format!("captch:account_{}", &arg.account.as_ref().unwrap()),
                captcha_str.as_str(),
            )
            .await;
        println!("{:?}", result);
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
