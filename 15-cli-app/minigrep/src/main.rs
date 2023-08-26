use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // in ok case unwrap
    // else problem parsing arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        // print to stderr
        eprintln!("Problem parsing Arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}