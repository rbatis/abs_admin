use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub database_struct: String,
}

fn main() {
    println!("start build");
    let js_data = include_str!("application.json5");
    let result: ApplicationConfig = json5::from_str(js_data).expect("load config file fail");
    println!("result={:?}", result);
    let mut f = File::create("target/driver.rs").unwrap();
    f.write_all(result.database_struct.as_ref()).unwrap();
    f.flush().unwrap();
    drop(f);
}