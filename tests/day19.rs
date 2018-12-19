extern crate day19;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5")
}

#[test]
fn day19_a() {
    let solver = day19::Day19::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "7");
}