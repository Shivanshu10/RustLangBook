fn main() {
    let s = String::from("hello world");
    println!("s: {}", s);
    let hello = get_hello(&s);
    println!("hello: {}", hello);
}

// slices help ref some porion without ref entire collection
// just like ref dont take ownership
fn get_hello(s: &String) -> &str {
    return &s[0..5];
}