struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        // return ref to x
        return &self.x;
    }
}

// only availble to f64 type of points
impl Point<f64> {
    fn y(&self) -> f64 {
        return self.y;
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    // p.y() will give error
    // p.y();

    let p = Point { x: 5.0, y: 10.0 };
    // p.y() will not give error
    p.y();
}