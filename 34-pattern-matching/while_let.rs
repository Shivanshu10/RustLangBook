fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // run loop as long as pattern specified contiue to match
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}