fn main() {
    let x = 5;
    let y = x; // copy stack variable directly
    println!("x: {}", x);
    println!("y: {}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    // no two owner of same thing
    // so it will move ownership from s1 to s2 and s1 doesnt exist latter
    println!("s2: {}", s2);
    // println!("s1: {}", s1);

    // clone copy, copies hey to other memory location in stack
    let s3 = String::from("hey");
    let s4 = s3.clone();

    println!("s3: {}", s3);
    println!("s4: {}", s4);
}