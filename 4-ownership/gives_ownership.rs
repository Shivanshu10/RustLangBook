fn main() {
    let s1 = givesOwnership();
    println!("s1: {}", s1);
}

fn givesOwnership() -> String {
    let some_string = String::from("hello");
    return some_string;
}