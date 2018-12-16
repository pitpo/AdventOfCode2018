extern crate day17;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day17_a() {
    let solver = day17::Day17::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day17_b() {
    let solver = day17::Day17::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}