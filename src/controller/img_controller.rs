use actix_web::{web, Responder, HttpResponse};
use crate::domain::dto::CatpchaDTO;

use captcha::Captcha;
use captcha::filters::{Noise, Wave, Dots};

///用户角色分页
pub async fn captcha(arg: web::Query<CatpchaDTO>) -> impl Responder {

    println!("account:{}",arg.account.as_ref().unwrap_or(&"".to_string()));
    let png=Captcha::new()
    .add_chars(4)
    .apply_filter(Noise::new(0.1))
    .apply_filter(Wave::new(1.0, 10.0).horizontal())
    // .apply_filter(Wave::new(2.0, 20.0).vertical())
    .view(160, 60)
    .apply_filter(Dots::new(4))
    .as_png();

    HttpResponse::Ok().content_type("image/png").body(png.unwrap())
}