fn main() {
    let sum = my_function(2, 3);
    println!("sum: {}", sum);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("x: {}", x);
    println!("y: {}", y);
    let sum = x + y;
    return sum;
}