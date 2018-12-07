extern crate day7;
extern crate utils;

use utils::Day;

#[test]
fn day7_a() {
    let solver = day7::Day7::new(String::from("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "CABDFE");
}

#[test]
fn day7_b() {
    let solver = day7::Day7::new(String::from("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "15");
}
