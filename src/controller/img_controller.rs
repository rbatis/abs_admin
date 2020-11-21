use actix_web::{web, Responder, HttpResponse};
use crate::domain::dto::CatpchaDTO;

use captcha::Captcha;
use captcha::filters::{Noise, Wave, Dots};

///用户角色分页
pub async fn captcha(arg: web::Path<CatpchaDTO>) -> impl Responder {

    let png=Captcha::new()
    .add_chars(5)
    .apply_filter(Noise::new(0.4))
    .apply_filter(Wave::new(2.0, 20.0).horizontal())
    .apply_filter(Wave::new(2.0, 20.0).vertical())
    .view(220, 120)
    .apply_filter(Dots::new(15))
    .as_png();

    HttpResponse::Ok().content_type("image/png").body(png.unwrap())
}