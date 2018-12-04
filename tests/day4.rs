extern crate day4;
extern crate utils;

use utils::Day;

#[test]
fn day4_a() {
    let solver = day4::Day4::new(String::from(
        "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-02 00:50] wakes up
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
    ));

    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "240");
}

#[test]
fn day4_b() {
    let solver = day4::Day4::new(String::from(
        "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-02 00:50] wakes up
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
    ));

    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "4455");
}
