pub struct User {
    name: String,
    age: u32,
    book: String,
}

impl User {
    pub fn create(name: &str, age: u32) -> Self {
        User {
            name: String::from(name),
            age,
            book: String::from(""),
        }
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Borrowed Book: {}", self.book);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn borrow(&mut self, name: &str) {
        self.book = String::from(name);
    }

    pub fn return_book(&mut self) {
        self.book = String::from("");
    }
}
