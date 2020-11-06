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
    use abs_admin::util::bencher::QPS;


    #[async_std::test]
    pub async fn test_http_req() {
        let resp = reqwest::get("http://127.0.0.1:8000").await.unwrap();
        let data = resp.bytes().await.unwrap();
        println!("data:{:#?}", String::from_utf8(data.to_vec()).unwrap());
    }

    //use Time: 4.3116499s ,each:431164 ns/op
    #[async_std::test]
    pub async fn bench_http_req() {
        let client = reqwest::Client::new();
        let resp = client.get("http://127.0.0.1:8000").send().await.unwrap();
        let data = resp.bytes().await.unwrap();
        println!("data:{:#?}", String::from_utf8(data.to_vec()).unwrap());
        let total = 10000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            let resp = client.get("http://127.0.0.1:8000").send().await.unwrap();
            let data = resp.bytes().await.unwrap();
        }
        now.time(total);
    }

    //use Time: 4.8274562s ,each:482745 ns/op
    #[test]
    pub fn bench_http_block_req() {
        let client = reqwest::blocking::Client::new();
        let resp = client.get("http://127.0.0.1:8000").send().unwrap();
        let data = resp.bytes().unwrap();
        println!("data:{:#?}", String::from_utf8(data.to_vec()).unwrap());
        let total = 10000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            let resp = client.get("http://127.0.0.1:8000").send().unwrap();
            let data = resp.bytes().unwrap();
        }
        now.time( total);
    }
}
