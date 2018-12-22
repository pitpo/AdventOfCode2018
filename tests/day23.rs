extern crate day23;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day23_a() {
    let solver = day23::Day23::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day23_b() {
    let solver = day23::Day23::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}