#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::boxes::List::{Cons, Nil};

pub fn run() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
}
