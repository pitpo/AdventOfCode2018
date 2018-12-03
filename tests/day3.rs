extern crate day3;
extern crate utils;

use utils::Day;

#[test]
fn day3_a() {
    let solver = day3::Day3::new(String::from("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "4");
}

#[test]
fn day3_b() {
    let solver = day3::Day3::new(String::from("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"));
    
    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "3");
}