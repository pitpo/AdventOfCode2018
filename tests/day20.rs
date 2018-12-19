extern crate day20;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day20_a() {
    let solver = day20::Day20::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day20_b() {
    let solver = day20::Day20::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}