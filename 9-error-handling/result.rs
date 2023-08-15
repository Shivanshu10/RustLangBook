use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // match on error kind 
        Err(error) => match error.kind() {
            // if not found error then try creating a file and match on its result
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            }
            other_kind => panic!("Problem Opening the file error of kind: {:?}", other_kind)
        }
    };
}