pub trait IsEmptyString {
    fn is_empty(&self) -> bool;
}

impl IsEmptyString for Option<String> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => s.is_empty(),
            _ => true,
        };
    }
}

impl IsEmptyString for Option<&str> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => s.is_empty(),
            _ => true,
        };
    }
}
