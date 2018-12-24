extern crate day25;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day25_a() {
    let solver = day25::Day25::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "");
}

#[test]
fn day25_b() {
    let solver = day25::Day25::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}