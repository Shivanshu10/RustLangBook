use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// implement trait Hello Macro on Pancake
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
