extern crate day24;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4")
}

#[test]
fn day24_a() {
    let solver = day24::Day24::new(get_input());
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "5216");
}

// #[test]
// fn day24_b() {
//     let solver = day24::Day24::new(get_input());

//     let ans = solver.get_part_b_result();
    
//     assert_eq!(&ans, "51");
// }