extern crate day17;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504")
}

#[test]
fn day17_a() {
    let solver = day17::Day17::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "57");
}

#[test]
fn day17_b() {
    let solver = day17::Day17::new(get_input());

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}