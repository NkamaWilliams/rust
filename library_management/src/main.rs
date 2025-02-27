pub mod books;
pub mod users;

use books::*;
use users::*;
fn main() {
    println!("Hello, world!");
    let user1 = User::create("Nkama Williams", 21);
    let book = Book::create("To Kill a Mocking Bird");
    user1.display();
}
