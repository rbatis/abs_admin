use crate::domain::dto::CatpchaDTO;
use crate::error_info;
use crate::context::CONTEXT;
use crate::util::string::IsEmptyString;
use axum::body::Body;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;

/// Image Code interface
/// Http Method GET
/// example：
/// http://localhost:8000/admin/captcha?account=18900000000
///
pub async fn captcha(arg: Query<CatpchaDTO>) -> impl IntoResponse {
    if arg.account.is_empty() {
        let resp = Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "json")
            .body(Body::from(error_info!("account_empty")))
            .unwrap();
        return resp;
    }
    let (png, code) = make();
    if CONTEXT.config.debug() {
        log::info!("account:{},captcha:{}", arg.account.as_ref().unwrap(), code);
    }
    if arg.account.is_some() {
        let result = CONTEXT
            .cache_service
            .set_string(
                &format!("captch:account_{}", arg.account.as_ref().unwrap()),
                code.as_str(),
            )
            .await;
        println!("{:?}", result);
        if CONTEXT.config.debug() == false {
            //release mode, return the error
            if let Err(e) = result {
                let resp = Response::builder()
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Cache-Control", "no-cache")
                    .header("Content-Type", "json")
                    .body(Body::from(e.to_string()))
                    .unwrap();
                return resp;
            }
        }
    }
    let resp = Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "no-cache")
        .header("Content-Type", "image/png")
        .body(Body::from(png))
        .unwrap();
    resp.into()
}

fn make() -> (Vec<u8>, String) {
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
    (png, captcha_str)
}
