// default all thread run in seprate thread
// print by default not seen in passing case
// unit test same file as product code
// intergation test in /tests
// integration test only after unit tests
// run only integration test
// file 
// integration test common for libs
// bin crate just wrapper around lib

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

fn fail() {
    panic!("I panicked!!!!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn small_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger), "put custom error msg here {}", "placeholders");
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    // assrt code to panic
    #[should_panic]
    fn should_panic() {
        fail();
    }

    #[test]
    fn it_Works() -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        } else {
            return Err(String::from("two plus two doesnt equal to four"))
        }
    }

    #[test]
    fn example() {
        let a = 3;
        let b = 1 + 2;

        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }
}
