fn add_one(x: i32) -> i32 {
    return x + 1;
}

// take any func ptr or closure
fn do_twice<T>(f: T, arg: i32) -> i32
where T: Fn(i32) -> i32 {
    return f(arg) + f(arg);
}

fn main() {
    let ans = do_twice(add_one, 5);

    println!("ans = {}", ans);
}