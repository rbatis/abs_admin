pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        match self {
            Some(s) => {
                return s.is_empty();
            }
            _ => {
                return true;
            }
        }
    }
}
