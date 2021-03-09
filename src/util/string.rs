pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Option<String> {
    fn is_empty(&self) -> bool {
        self.as_ref().map_or_else(
            || false,
            |v| {
                if v.is_empty() {
                    true
                } else {
                    false
                }
            },
        )
    }
}
