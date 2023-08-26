pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // vector of trait objects, any object that implements trait Draw
    // dynamic calls
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing Button")
    }
}