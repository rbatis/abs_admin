use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
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
    unwrap_check("src/controller");
    unwrap_check("src/domain/dto");
    unwrap_check("src/domain/vo");
    unwrap_check("src/middleware");
    unwrap_check("src/service");
    unwrap_check("src/util");
}

//format print
fn emit_rust_error(path: &str, line_no: usize, col: usize, line: &str, msg: &str) {
    println!("error: {}", msg);
    println!("  --> {}:{}:{}", path, line_no, col);
    println!("   |");
    println!("{:>3} | {}", line_no, line);
    // underline line
    print!("   | ");
    // print (col-1) spaces
    for _ in 0..col {
        print!(" ");
    }
    // caret underline
    println!("^---- {}", msg);
}

//check server code have .unwrap()
fn unwrap_check(dir: &str) {
    for entry in WalkDir::new(dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let path_str = path.to_string_lossy().to_string();

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let reader = BufReader::new(file);

        for (line_no, line) in reader.lines().enumerate() {
            let line_no = line_no + 1;
            let line = line.unwrap_or_default();

            if let Some(col) = line.find(".unwrap()") {
                emit_rust_error(&path_str, line_no, col + 1, &line, "found .unwrap()");
                std::process::exit(1);
            }

            if let Some(col) = line.find("panic!(") {
                emit_rust_error(&path_str, line_no, col + 1, &line, "found panic!()");
                std::process::exit(1);
            }

            if let Some(col) = line.find(".expect(") {
                emit_rust_error(&path_str, line_no, col + 1, &line, "found .expect()");
                std::process::exit(1);
            }
        }
    }
}
