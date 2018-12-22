extern crate day22;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("depth: 510
target: 10,10")
}

#[test]
fn day22_a() {
    let solver = day22::Day22::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "114");
}

#[test]
fn day22_b() {
    let solver = day22::Day22::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "45");
}