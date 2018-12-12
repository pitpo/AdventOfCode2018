extern crate day12;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #")
}

#[test]
fn day12_a() {
    let solver = day12::Day12::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "325");
}
