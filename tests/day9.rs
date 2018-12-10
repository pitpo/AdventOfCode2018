extern crate day9;
extern crate utils;

use utils::Day;

#[test]
fn day9_a0() {
    let solver = day9::Day9::new(String::from("9 players; last marble is worth 25 points"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "32");
}

#[test]
fn day9_a1() {
    let solver = day9::Day9::new(String::from("10 players; last marble is worth 1618 points"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "8317");
}

#[test]
fn day9_a2() {
    let solver = day9::Day9::new(String::from("13 players; last marble is worth 7999 points"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "146373");
}

#[test]
fn day9_a3() {
    let solver = day9::Day9::new(String::from("17 players; last marble is worth 1104 points"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "2764");
}

#[test]
fn day9_a4() {
    let solver = day9::Day9::new(String::from("21 players; last marble is worth 6111 points"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "54718");
}

#[test]
fn day9_a5() {
    let solver = day9::Day9::new(String::from("30 players; last marble is worth 5807 points"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "37305");
}
