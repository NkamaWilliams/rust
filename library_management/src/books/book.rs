pub struct Book {
    pub name: String,
    is_available: bool,
    borrowed_by: String,
}

impl Book {
    pub fn create(name: &str) -> Self {
        Book {
            name: String::from(name),
            is_available: true,
            borrowed_by: String::from(""),
        }
    }

    pub fn available(&self) -> bool {
        self.is_available
    }

    pub fn borrower(&self) -> &str {
        &self.borrowed_by
    }
}
