
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: &str) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write(raw_password.as_bytes());
        let result = hasher.finish().to_string();

        let mut new_result = String::new();
        for x in result.chars() {
            match x {
                '0' => {
                    new_result.push('a');
                }
                '1' => {
                    new_result.push('b');
                }
                '2' => {
                    new_result.push('c');
                }
                '3' => {
                    new_result.push('d');
                }
                '4' => {
                    new_result.push('e');
                }
                '5' => {
                    new_result.push('f');
                }
                '6' => {
                    new_result.push('g');
                }
                '7' => {
                    new_result.push('h');
                }
                '8' => {
                    new_result.push('i');
                }
                '9' => {
                    new_result.push('j');
                }
                _ => {
                    new_result.push('0');
                }
            }
        }

        new_result
    }
    pub fn verify(password: &str, raw_password: &str) -> bool {
        let hashed = PasswordEncoder::encode(raw_password);
        password.eq(&hashed)
    }
}

#[test]
fn test_encode() {
    let s = PasswordEncoder::encode("123456");
    println!("{}", s);
    assert_eq!(PasswordEncoder::encode("123456"), PasswordEncoder::encode("123456"))
}