pub trait Iterator {
    // some type Item
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}


// during implementation specify type
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(0);
    }
}

// not possible
// cause one implemetation already for this implemetation which has item type of u32
/*
impl Iterator for Counter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(0);
    }
}
 */