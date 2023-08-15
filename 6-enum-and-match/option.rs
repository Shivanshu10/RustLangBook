fn main() {
    let x = 5;
    let some_number = Some(5);
    let some_string = Some("a string");

    // have to annotate type for none
    let absent_number: Option<i32> = None;

    // unwrap will do normal for ok/some case if not ok/some type enum it will panic
    // unwrap_or, will not panic instead it will use default value
    let sum = x + some_number.unwrap_or(0);

    println!("sum: {}", sum);
}