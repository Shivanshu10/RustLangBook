use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // called when deref
    fn deref(&self) -> &T {
        return &self.0;
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // deref implemented for MyBox so can use deref
    assert_eq(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // even tho m is of type mybox, rust automatically deref it
    // &MyBox<String> -> &String -> &str
    hello(m);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}