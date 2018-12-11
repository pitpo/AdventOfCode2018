extern crate day11;
extern crate utils;

use utils::Day;

#[test]
fn day11_a1() {
    let solver = day11::Day11::new(String::from("18"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "33,45");
}


#[test]
fn day11_a2() {
    let solver = day11::Day11::new(String::from("42"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "21,61");
}

#[test]
fn day11_b1() {
    let solver = day11::Day11::new(String::from("18"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "90,269,16");
}

#[test]
fn day11_b2() {
    let solver = day11::Day11::new(String::from("42"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "232,251,12");
}