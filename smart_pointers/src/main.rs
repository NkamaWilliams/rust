mod cons_list;
mod limit;
use cons_list::{
    List::{Cons, Nil},
    Message, MyBox, Winner,
};
use limit::*;
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    let b = Cons(0, Rc::clone(&a));
}
