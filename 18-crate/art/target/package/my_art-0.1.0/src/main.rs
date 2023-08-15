// tied to internal module struct
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// much easy
use art::PrimaryColor;
use arg::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}