extern crate day5;
extern crate utils;

use utils::Day;

#[test]
fn day5_a() {
    let solver = day5::Day5::new(String::from("dabAcCaCBAcCcaDA"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "10");
}

#[test]
fn day5_b() {
    let solver = day5::Day5::new(String::from("dabAcCaCBAcCcaDA",));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "4");
}
