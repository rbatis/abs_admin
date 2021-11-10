pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => {
                s.is_empty()
            }
            _ => {
                true
            }
        }
    }
}
