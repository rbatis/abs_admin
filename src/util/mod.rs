use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: &str) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write(raw_password.as_bytes());
        hasher.finish().to_string()
    }
    pub fn verify(password: &str, raw_password: &str) -> bool {
        let hashed= PasswordEncoder::encode(raw_password);
        password.eq(&hashed)
    }
}

#[test]
fn test_encode() {
    let s = PasswordEncoder::encode("123456");
    println!("{}", s);
    assert_eq!(PasswordEncoder::encode("123456"), PasswordEncoder::encode("123456"))
}