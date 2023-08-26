fn main() {
    // unsafe func
    // must call inside other unsafe func or unsafe block
    // unsafe func no need internal safe block
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}