// macro to auto impelement debug trait
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

// house methods for struct
impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

// associated function or static function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            height: size,
            width: size
        };
    }
}

fn main() {
    let rect = Rectangle{
        height: 2,
        width: 3
    };

    let rect2 = Rectangle{
        height: 10,
        width: 20
    };

    // use debug trait to print structs
    println!("rect: {:?}", rect);

    println!("area: {}", rect.area());

    println!("can contain: {}", rect2.can_hold(rect));

    println!("Square: {:?}", Rectangle::square(2));
}