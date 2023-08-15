fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("s ownership destroyed, after function returns")
    // println!("{}", s);
}

fn takes_ownership(some_string: String) {
    println!("takes ownership of {}", some_string);
}