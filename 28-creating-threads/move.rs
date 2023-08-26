use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // new thread need outside variable access, but it may be deleted later, so use move
        // so thread has ownership of v
        println!("Here's a vector {:?}", v);
    });

    handle.join().unwrap();
}