use adder;
mod common;
#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(6,adder::add(2,4));
}