extern crate day14;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day14_a() {
    let solver = day14::Day14::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day14_b() {
    let solver = day14::Day14::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}