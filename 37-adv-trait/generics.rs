pub trait Iterator<T> {
    // some type Item
    type T;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}


impl Iterator<u32> for Counter {
    
    fn next(&mut self) -> Option<Self::Item> {
        return Some(0);
    }
}

// possible
impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<Self::Item> {
        return Some(0);
    }
}