use std::fs::{OpenOptions};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub database_struct: String,
}

fn main() {
    println!("start build");
    let js_data = include_str!("application.json5");
    let config: ApplicationConfig = json5::from_str(js_data).expect("load config file fail");
    println!("config={:?}", config);
    let mut data = String::new();
    let mut f = OpenOptions::new().write(true).create(true).open("target/driver.rs").unwrap();
    _ = f.read_to_string(&mut data);
    if data.is_empty() || data!=config.database_struct{
        _ = f.set_len(0);
        f.write_all(config.database_struct.as_ref()).unwrap();
        f.flush().unwrap();
    }
}