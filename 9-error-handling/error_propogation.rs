use std::fs:File;
use std::io;
use std::io::Read;

// calling function will deal with error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match {
        Ok(file) => file,
        Err(e) => return Err(e),
    }
}