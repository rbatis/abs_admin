pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return true;
                } else {
                    return false;
                }
            }
            _ => { return true; }
        }
    }
}
