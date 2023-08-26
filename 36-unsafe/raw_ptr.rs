fn main() {
    let mut num = 5;

    // raw ptrs
    // ignore rust borrowing rule
    // can have deref
    // no auto cleanup 
    // immutable ptr
    // convert ref tp immutable raw ptr
    let r1 = &num as *const i32;
    // mutable ptr
    let r2 = &mut num as *mut i32;

    // point to raw addr
    let addr = 0x012345usize;
    let r3 = addr as *const i32;

    // cannot deref raw ptr without unsafe block
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}