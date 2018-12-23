extern crate day24;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day24_a() {
    let solver = day24::Day24::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day24_b() {
    let solver = day24::Day24::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}