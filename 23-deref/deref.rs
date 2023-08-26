fn main() {
    let x = 5;
    let y = &x;

    assert_eq(5, x);
    // deref y ptr
    assert_eq(5, *y);
    // not allowed
    // assert_eq(5, y);
}