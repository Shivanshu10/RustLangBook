fn main() {
    let mut s1 = String::from("hello");

    // only one mutable ref in a scope
    let r1 = &mut s1;
    // not allowed
    // let r2 = &mut s1;


    let mut s2 = String::from("s2");
    // if mut then no immutable ref in scope
    let r2 = &mut s2;
    // not allowed
    // let r3 = &s2;

    let mut s3 = String::from("s3");
    // multiple immutable allowed
    let r4 = &s3;
    let r5 = &s3;
    println!("r3: {}, r4: {}", r4, r5);
}