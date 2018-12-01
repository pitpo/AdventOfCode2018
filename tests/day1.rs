extern crate day1;
extern crate utils;

use utils::Day;

#[test]
fn day1_example1() {
    let solver = day1::Day1::new(String::from("+1\n-2\n+3\n+1"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "3");
}

#[test]
fn day1_example2() {
    let solver = day1::Day1::new(String::from("+7\n+7\n-2\n-7\n-4"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "14");
}