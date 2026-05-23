use crate::context::CONTEXT;
use crate::domain::dto::CatpchaDTO;
use crate::error_info;
use crate::util::string::IsEmptyString;
use actix_web::web;
use actix_web::HttpResponse;
use captcha::Captcha;
use captcha::filters::{Dots, Noise, Wave};

pub async fn captcha(arg: web::Query<CatpchaDTO>) -> HttpResponse {
    if arg.account.is_empty() {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .content_type("json")
            .body(error_info!("account_empty"));
    }
    let (png, code) = make();
    if CONTEXT.config.debug() {
        log::info!(
            "account:{},captcha:{}",
            arg.account.as_deref().unwrap_or_default(),
            code
        );
    }
    if arg.account.is_some() {
        let result = CONTEXT
            .cache_service
            .set_string(
                &format!(
                    "captch:account_{}",
                    arg.account.as_deref().unwrap_or_default()
                ),
                code.as_str(),
            )
            .await;
        if CONTEXT.config.debug() == false {
            if let Err(e) = result {
                return HttpResponse::Ok()
                    .insert_header(("Access-Control-Allow-Origin", "*"))
                    .insert_header(("Cache-Control", "no-cache"))
                    .content_type("json")
                    .body(e.to_string());
            }
        }
    }
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .content_type("image/png")
        .body(png)
}

fn make() -> (Vec<u8>, String) {
    let mut captcha = Captcha::new();
    captcha
        .add_chars(4)
        .apply_filter(Noise::new(0.1))
        .apply_filter(Wave::new(1.0, 10.0).horizontal())
        .view(160, 60)
        .apply_filter(Dots::new(4));
    let png = captcha.as_png().unwrap_or_default();
    let captcha_str = captcha.chars_as_string().to_lowercase();
    (png, captcha_str)
}
