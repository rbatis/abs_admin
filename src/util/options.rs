use once_cell::sync::Lazy;

pub trait OptionStringRefUnwrapOrDefault<T> {
    fn unwrap_or_default(&self) -> T;
}

static EMPTY_STR: Lazy<String> = Lazy::new(|| {
    String::new()
});


impl<'a> OptionStringRefUnwrapOrDefault<&'a String> for Option<&'a String> {
    fn unwrap_or_default(&self) -> &'a String {
        return match self {
            None => { &EMPTY_STR }
            Some(v) => {
                v
            }
        };
    }
}
