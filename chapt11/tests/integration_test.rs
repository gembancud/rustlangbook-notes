use chapt11;
mod common;

#[test]
fn chapt11_add() {
    common::setup();
    assert_eq!(2, chapt11::add(2, 0));
}
