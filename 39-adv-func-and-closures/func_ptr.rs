fn add_one(x: i32) -> i32 {
    return x + 1;
}

// lowercase fn means func ptr
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    return f(arg) + f(arg);
}

fn main() {
    let ans = do_twice(add_one, 5);

    println!("ans = {}", ans);
}