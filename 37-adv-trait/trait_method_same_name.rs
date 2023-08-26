trait Pilot {
    fn fly(&self)
}

trait Wizard {
    fn fly(&self)
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("waving arms");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up");
    }
}

fn main() {
    let person = Human;
    person.fly();
    // call fly method for pilot trait
    Pilot::fly(&person);
}