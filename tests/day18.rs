extern crate day18;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from(".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.")
}

#[test]
fn day18_a() {
    let solver = day18::Day18::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "1147");
}