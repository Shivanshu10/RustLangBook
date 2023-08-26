enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // b now owns a
    let b = Cons(3, Box::new(a));
    // not allowed cannot have multiple owner of a
    let c = Cons(4, Box::new(a));
}