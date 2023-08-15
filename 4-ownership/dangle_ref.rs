fn main() {
    let ref_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    // not allowed lifetime error
    return &s;
}