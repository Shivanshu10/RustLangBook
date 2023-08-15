// nested import
// self to bring current into scope
use rand::{self, Rng, CrytoRng, ErrorKind::Transient};

// import all
use std::io::*;

// declaration 
mod fron_of_house;

// bring to scope
pub use crate::fron_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}