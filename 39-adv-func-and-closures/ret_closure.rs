fn main() {

}

// return something that implements Fn trait
fn return_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        move | b | a + b;
    } else {
        move | b | a - b;
    }
}