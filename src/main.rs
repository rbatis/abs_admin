use actix_web::{App, HttpResponse, HttpServer, Responder, web};

use abs_admin::config::CONFIG;
use abs_admin::controller::{res_controller, role_controller, user_controller};
use abs_admin::dao::RB;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello ! Please use Post(Json) request /login,/role_page,/res_page....more http interface,you can install postman for import postman.json ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    abs_admin::config::log::init_log();
    //ORM
    RB.link(&CONFIG.mysql_url).await.unwrap();
    //路由
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            //TODO .route("/res_update", web::post().to(res_controller::res_update))
            //TODO .route("/res_delete", web::post().to(res_controller::res_delete))
            .route("/res_add", web::post().to(res_controller::add))
            .route("/res_page", web::post().to(res_controller::page))
            .route("/login", web::post().to(user_controller::login))
            //TODO .route("/log_out", web::post().to(user_controller::log_out))
            .route("/user_add", web::post().to(user_controller::add))
            .route("/user_page", web::post().to(user_controller::page))
            //TODO .route("/user_delete", web::post().to(user_controller::user_delete))
            //TODO .route("/user_update", web::post().to(user_controller::user_update))
            .route("/role_add", web::post().to(role_controller::add))
            //TODO .route("/role_update", web::post().to(role_controller::role_update))
            //TODO .route("/role_delete", web::post().to(role_controller::role_delete))
            .route("/role_page", web::post().to(role_controller::page))
            //TODO .route("/user_role_add", web::post().to(role_controller::user_role_add))
            //TODO .route("/user_role_delete", web::post().to(role_controller::user_role_delete))
            //TODO .route("/user_role_update", web::post().to(role_controller::user_role_update))
            //TODO .route("/user_role_page", web::post().to(role_controller::user_role_page))
            //TODO .route("/captcha", web::post().to(img_controller::captcha))
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use serde::de::DeserializeOwned;
    use serde_json::json;

    //post get string
    pub async fn post(path: &str, arg: &serde_json::Value) -> String {
        let client = reqwest::Client::new();
        println!("body:{}", arg.to_string());
        let resp = client.post(&format!("http://127.0.0.1:8000{}", path))
            .header("content-type", "json")
            .json(arg)
            .send().await.unwrap();
        let data = resp.bytes().await.unwrap();
        let data = String::from_utf8(data.to_vec()).unwrap();
        println!("data:{:#?}", &data);
        data
    }

    //post get json
    pub async fn post_json<R>(path: &str, arg: &serde_json::Value) -> R where R: DeserializeOwned {
        serde_json::from_str(&post(path, arg).await).unwrap()
    }

    #[async_std::test]
    pub async fn test_res_page() {
        let v: serde_json::Value = post_json("/res_page", &json!({ })).await;
        println!("{:#?}", v);
    }
}
