use crypto::bcrypt::bcrypt;
use std::fmt::Write;

pub struct BCryptPasswordEncoder {}

impl BCryptPasswordEncoder {
    pub fn encode(raw_password: &str) -> String {
        let mut output = [0u8; 24];
        let salt = vec![0x10u8, 0x41u8, 0x04u8, 0x10u8, 0x41u8, 0x04u8, 0x10u8, 0x41u8, 0x04u8, 0x10u8, 0x41u8, 0x04u8, 0x10u8, 0x41u8, 0x04u8, 0x10u8];
        bcrypt(5, &salt, (&raw_password).as_ref(), &mut output[..]);

        let mut output_password = String::new();
        for a in output.iter() {
            write!(output_password, "{:02x}", a);
        }
        return output_password;
    }
    pub fn verify(password: &str, raw_password: &str) -> bool {
        let s = BCryptPasswordEncoder::encode(raw_password);
        password.eq(&s)
    }
}

#[test]
fn test_encode() {
    let s = BCryptPasswordEncoder::encode("123456");
    println!("{}", s);
}