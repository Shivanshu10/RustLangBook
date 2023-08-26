use std::ops::Add;

// implementation of Add trait
// has default value of Rhs, cause generally add same types together
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

fn main() {
    assert_eq!(
        Point {x: 1, y: 0} + Point {x: 0, y: 1},
        Point {x: 1, y: 1}
    );
}