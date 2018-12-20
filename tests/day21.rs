extern crate day21;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day21_a() {
    let solver = day21::Day21::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day21_b() {
    let solver = day21::Day21::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}