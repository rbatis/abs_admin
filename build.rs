/*
 * @Date: 2024-05-21 21:36:59
 * @LastEditTime: 2024-05-21 22:28:23
 */

use std::fs::OpenOptions;
use std::io::{Read, Write};

//choose driver struct(Cargo.toml must add like 'rbdc-*** = { version = "4.5" }')
//database_struct: "rbdc_sqlite::Driver{}",
//database_struct: "rbdc_mysql::Driver{}",
//database_struct: "rbdc_mssql::Driver{}",
//database_struct: "rbdc_pg::Driver{}",
//database_struct: "rbdc_sqlite::Driver{}",
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub db_url: String,
}

/// write `rbdc_<database>::Driver{}` to file 'target/driver.rs'
fn main() {
    let js_data = include_str!("application.json5");
    let config: ApplicationConfig = json5::from_str(js_data).expect("load config file fail");
    let mut data = String::new();
    let mut f = OpenOptions::new()
        .write(true)
        .create(true).truncate(true)
        .open("target/driver.rs")
        .unwrap();
    _ = f.read_to_string(&mut data);

    let db_index = config
        .db_url
        .find(':')
        .expect("db_url must be '<database>://xxxx'");
    let mut db_name = &config.db_url[..db_index];
    if db_name == "postgres" {
        db_name = "pg";
    }
    let driver_path = format!("rbdc_{}::Driver{}", db_name, "{}");
    println!("driver_path={}", driver_path);
    _ = f.set_len(0);
    f.write_all(driver_path.as_bytes()).unwrap();
    f.flush().unwrap();
}
