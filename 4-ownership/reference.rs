fn main() {
    let mut s1 = String::from("hello");
    println!("prev s1: {}", s1);
    // referebce gives temp access
    // mutable ref
    change_string(&mut s1);
    println!("curr s1: {}", s1);
    // referebce gives temp access
    // immutable ref
    let len = get_length(&s1);
    println!("The length of {}, is {}", s1, len);
}

// referebce gives temp access
fn get_length(s: &String) -> usize {
    // ref are immuntable by default
    // s.push_str("oops");
    return s.len();
}

fn change_string(s: &mut String) {
    // now ref is mutable
    s.push_str(", world");
}