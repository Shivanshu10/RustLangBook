use std::io;
use rand::Rng;
// result of two things being compared, enum
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the Number");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("Secret Number is {}", secret_num);

    loop {
        
        println!("Please input a number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Line");

        // parse function returns and result type
        // which might fail, it fail u can provide custom error msg in expect
        // if not fail it will unweap
        // unwrap will only unwrap in case of Ok, else panic
        // note we are here redeclaring variable with same name, but diff type this is called shadowing
        // let guess: u32 = guess.trim().parse().expect("Failed to convert");

        // avoid fail on text input
        // match will return num if ok type enum and error of any type will continue to ask for input again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed {}", guess);

        // multiple statements in a block
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Two Small".red()),
            Ordering::Equal => {
                println!("{}", "You Win!!".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too Big".red())
        }
    }
}
