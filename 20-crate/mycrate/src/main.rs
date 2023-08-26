//! # Documentation for item containing comment
//!  markdown for formatting
//! use cargo doc --open, to build doc

fn main() {
    println!("Hello, world!");
}

/// # Examples, documents code following comment
/// use cargo test to run example test
/// ```
/// let arg = 5;
/// let answer = mycrate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
fn add_one(arg: i32) -> i32 {
    return arg + 1;
}