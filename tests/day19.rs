extern crate day19;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("")
}

#[test]
fn day19_a() {
    let solver = day19::Day19::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "1147");
}

#[test]
fn day19_b() {
    let solver = day19::Day19::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}