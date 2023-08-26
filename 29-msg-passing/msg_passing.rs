// multiple producer and single rcvr channel
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); 

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
        // cannot use cause channel took ownership of msg
        // println!("Message Is: {}", msg);
    });


    // block until has rcvd anything
    let recvd = rx.recv().unwrap();
    // try_recv not block execution
    // generally used in look
    // let recvd = rx.try_recv().unwrap();
    println!("Got: {}", recvd);
}