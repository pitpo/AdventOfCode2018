extern crate day15;
extern crate utils;

use utils::Day;

#[test]
fn day15_a1() {
    let solver = day15::Day15::new(String::from("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "27730");
}

#[test]
fn day15_a2() {
    let solver = day15::Day15::new(String::from("#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "36334");
}

#[test]
fn day15_a3() {
    let solver = day15::Day15::new(String::from("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "39514");
}

#[test]
fn day15_a4() {
    let solver = day15::Day15::new(String::from("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "27755");
}

#[test]
fn day15_a5() {
    let solver = day15::Day15::new(String::from("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "28944");
}

#[test]
fn day15_a6() {
    let solver = day15::Day15::new(String::from("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "18740");
}

#[test]
fn day15_b1() {
    let solver = day15::Day15::new(String::from("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"));
    
    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "4988");
}

#[test]
fn day15_b3() {
    let solver = day15::Day15::new(String::from("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "31284");
}

#[test]
fn day15_b4() {
    let solver = day15::Day15::new(String::from("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "3478");
}

#[test]
fn day15_b5() {
    let solver = day15::Day15::new(String::from("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "6474");
}

#[test]
fn day15_b6() {
    let solver = day15::Day15::new(String::from("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "1140");
}
