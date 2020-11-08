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
            .route("/res_add", web::post().to(res_controller::add))
            .route("/res_page", web::post().to(res_controller::page))
            .route("/login", web::post().to(user_controller::login))
            .route("/user_add", web::post().to(user_controller::add))
            .route("/user_page", web::post().to(user_controller::page))
            .route("/role_add", web::post().to(role_controller::add))
            .route("/role_page", web::post().to(role_controller::page))
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use serde::de::DeserializeOwned;
    use serde_json::json;

    use abs_admin::util::bencher::QPS;

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
