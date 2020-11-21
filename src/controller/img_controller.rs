use actix_web::{HttpResponse, Responder, web};
use captcha::Captcha;
use captcha::filters::{Dots, Noise, Wave};

use crate::domain::dto::CatpchaDTO;
use crate::service::REDIS_SERVICE;

///用户角色分页
pub async fn captcha(arg: web::Query<CatpchaDTO>) -> impl Responder {
    let mut captcha = Captcha::new();
    captcha.add_chars(4)
        .apply_filter(Noise::new(0.1))
        .apply_filter(Wave::new(1.0, 10.0).horizontal())
        // .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(160, 60)
        .apply_filter(Dots::new(4));
    let png = captcha.as_png().unwrap();

    let captcha_str = captcha.chars_as_string();
    println!("account:{},captcha:{}", arg.account.as_ref().unwrap_or(&"".to_string()), &captcha_str);
    if arg.account.is_some() {
        let result=REDIS_SERVICE.set_string(&format!("captch:account-{}", &arg.account.as_ref().unwrap()), captcha_str.as_str()).await;
        println!("{:?}",result);
    }
    HttpResponse::Ok().content_type("image/png").body(png)
}