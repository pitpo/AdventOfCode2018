extern crate day11;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day11_a() {
    let solver = day11::Day11::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day11_b() {
    let solver = day11::Day11::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}