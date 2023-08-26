use std::fmt;

// implement display trait for vec type
// but both outside defiined has to bypass orphan trait

// sol
// wrap around display trait
// but wrapper has to implment every func for vec trait
// sol implement deref trait, deref wrapper will return vec
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    println!("w = {}", w);
}