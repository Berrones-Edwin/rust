extern crate adder;

#[test]
fn it_add_two_integration() {
    assert_eq!(4, adder::add_two(2));
}
