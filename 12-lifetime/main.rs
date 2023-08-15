use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Annoucement: {}", ann);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn main() {
}