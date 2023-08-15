fn main() {
    println!("Variable are immuntable by default");
    let x = 5;
    
    println!("The value of x is {}", x);

    // cannot reassign x cause it is immuntable by default
    // x = 6;

    println!("The value of x is {}", x);

    println!("Make them mutable");
    let mut y = 5;

    println!("The value of y is {}", y);

    y = 6;

    println!("The value of y is {}", y);

    println!("Use constants");
    // 100_000, is a naming convention in rust, which makes it easier to read
    // but actually it is 100000 onky
    const SUB_COUNT: u32 = 100_000;

    println!("Value of const SUB_COUNT is {}", SUB_COUNT);

    println!("Shadowing");
    let z: i32 = 5;
    println!("The value of z is {}", z);
    let z: i32 = 6;
    println!("The value of z is {}", z);

    println!("Compound Types: Tuple");
    let tup = ("0Let's Get Rusty", 100_000);
    
    println!("Destruct Tuple");
    let (channel, sub_count) = tup;
    println!("Channel: {}, Sub_Count: {}", channel, sub_count);

    println!("Get Specific Value");
    println!("Sub_Count: {}", tup.1);

    println!("Compound Types: Array");
    
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    println!("Not Found: {}", not_found);

    // create an array of 8 values all set 0
    let byte = [0; 8];
    println!("byte: {:?}", byte);
}