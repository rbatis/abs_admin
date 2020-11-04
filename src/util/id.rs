use std::time::{SystemTime, Instant};
use rustflake::Snowflake;
use std::sync::{Mutex, MutexGuard, TryLockError};

lazy_static!(
   pub static ref snowflake:Mutex<Snowflake> = Mutex::new(Snowflake::default());
);

pub fn new_id() -> i64 {
    loop {
        match snowflake.try_lock() {
            Ok(mut v) => {
                return v.generate();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::util::id::new_id;

    #[test]
    fn test_new_id() {
        println!("{}", new_id());
        println!("{}", new_id());
    }
}