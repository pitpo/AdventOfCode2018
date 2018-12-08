extern crate day8;
extern crate utils;

use utils::Day;

#[test]
fn day8_a() {
    let solver = day8::Day8::new(String::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "138");
}

#[test]
fn day8_b() {
    let solver = day8::Day8::new(String::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "66");
}