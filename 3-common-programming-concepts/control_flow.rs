fn main() {
    let number = 5;

    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("all condition was false");
    }

    println!("if else in let statement");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);
}