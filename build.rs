use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use walkdir::WalkDir;

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
        .create(true)
        .open("target/driver.rs")
        .unwrap();
    _ = f.read_to_string(&mut data);

    let db_index = config
        .db_url
        .find(":")
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

    //unwrap check
    unwrap_check();
}

//check server code have .unwrap()
fn unwrap_check() {
    let walk = WalkDir::new("src/service");
    for item in walk {
        if let Ok(item) = item {
            let path = item.path().to_str().unwrap_or_default();
            let name = item.file_name().to_str().unwrap_or_default();
            if name.ends_with(".rs") {
                if let Ok(mut f) = File::open(path) {
                    let mut data = String::new();
                    _ = f.read_to_string(&mut data);
                    if data.contains(".unwrap()") {
                        panic!("find file='{}' have .unwrap(),please check code", name);
                    }
                    if data.contains("panic!") {
                        panic!("find file='{}' have panic!(),please check code", name);
                    }
                    if data.contains(".expect(") {
                        panic!("find file='{}' have .expect(),please check code", name);
                    }
                }
            }
        }
    }
}
