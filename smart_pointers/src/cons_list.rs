use std::ops::Deref;
use std::rc::Rc;
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub struct MyBox<T>(T);
impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Winner {
    pub name: String,
}

impl Winner {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Drop for Winner {
    fn drop(&mut self) {
        println!("Dropping winner with name: {}", self.name);
    }
}
