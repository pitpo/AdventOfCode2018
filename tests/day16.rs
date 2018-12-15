extern crate day16;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day16_a() {
    let solver = day16::Day16::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day16_b() {
    let solver = day16::Day16::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}