// use generics here to take any func
struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut cacher = Cacher {
        calculation: |num| -> u32 {
            println!("Num is {}", num);
            return num;
        },
        value: None
    };

    cacher.value(2);
}