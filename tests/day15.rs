extern crate day15;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day15_a() {
    let solver = day15::Day15::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day15_b() {
    let solver = day15::Day15::new(get_input());
    
    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "");
}
