// store data with enum, here we are storing associated ip addr
// we can store diff type for each enum type
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    // stored anonymous struct
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_functon() {
        println!("Lets get rusty");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}