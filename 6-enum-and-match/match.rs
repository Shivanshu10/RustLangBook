enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
    Arizona,
    California
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    care_only_three(Some(3));
}

// only care for one arm instead of implementing arm for each possible value we can use _ for remaining
fn care_only_three(opt: Option<i32>) {
    match opt {
        Some(3) => println!("three"),
        _ => ()
    }
}

// match all possible results
fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}