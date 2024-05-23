pub struct PasswordEncoder {}

impl PasswordEncoder {
    /// Hash a password using bcrypt
    pub fn hash_password(password: impl AsRef<[u8]>) -> String {
        bcrypt::hash(password.as_ref(), bcrypt::DEFAULT_COST).unwrap_or_default()
    }

    pub fn md5(raw_password: impl AsRef<[u8]>) -> String {
        let digest = md5::compute(raw_password.as_ref());
        format!("{:x}", digest)
    }

    pub fn md5_and_hash(password: impl AsRef<[u8]>) -> String {
        let md5_password = PasswordEncoder::md5(password);
        PasswordEncoder::hash_password(md5_password)
    }

    pub fn verify(hash: &str, raw_password: &str) -> bool {
        if raw_password.eq(hash) {
            return true;
        }
        // let hashed = PasswordEncoder::encode(raw_password);
        // password.eq(&hashed)
        bcrypt::verify(raw_password, hash).unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use crate::util::password_encoder::PasswordEncoder;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::md5("123456");
        println!("{}", s);
        
        let s = PasswordEncoder::md5_and_hash("123456");
        println!("{}", s);
        assert_eq!(
            PasswordEncoder::md5("123456"),
            PasswordEncoder::md5("123456")
        )
    }

    #[test]
    fn test_verify() {
        let password = "12345";
        let raw_password = "12345";

        assert!(PasswordEncoder::verify(password, raw_password));

        let hash = PasswordEncoder::hash_password(password);
        assert!(PasswordEncoder::verify(&hash, password));
    }
}
