use adder; // root

mod common;


#[test]
fn it_adds_two_integration() {
    common::setup();
    common::printer::printer();
    assert_eq!(5, adder::add_two(2));
}