fn main() {
    let s = "hey";
    {
        let s = "hello";
        println!("s: {}", s);
    }
    println!("s is out of scope here, now s is: {}", s);
}