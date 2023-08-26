// foreign func abs
// C define which ABI foreign func use
extern "C" {
    fn abs(input: i32) => i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 as per C is: {}", abs(-3));
    }
}

// other lang to call c code
#[no_mangle]
pub extern "C" in call_from_c() {
    println!("Just called a rust funtion in C!!");
}