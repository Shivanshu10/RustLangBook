use std::slice;

// safe abstraction around unsafe code
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // get raw mut ptr of slice
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);

    unsafe {
        return (
            // start ptr and end size as param and returns a slice
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(
                ptr.add(mid), len - mid
            ),
        );
    }
}

fn main() {

}