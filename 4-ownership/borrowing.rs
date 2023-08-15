fn main() {
    let s1 = String::from("Hello");
    // referebce gives temp access
    let len = calculate_length(&s1);
    println!("The length of {}, is {}", s1, len);
}

// referebce gives temp access
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}