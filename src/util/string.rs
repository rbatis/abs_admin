pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        match self {
            Some(v) => {
                if v.is_empty() {
                    return true;
                }
                false
            }
            _ => true,
        }
    }
}
