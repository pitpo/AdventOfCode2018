extern crate day20;
extern crate utils;

use utils::Day;

#[test]
fn day20_a1() {
    let solver = day20::Day20::new(String::from("^ENWWW(NEEE|SSE(EE|N))$"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "10");
}

#[test]
fn day20_a2() {
    let solver = day20::Day20::new(String::from("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "18");
}

#[test]
fn day20_a3() {
    let solver = day20::Day20::new(String::from("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "23");
}

#[test]
fn day20_a4() {
    let solver = day20::Day20::new(String::from("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "31");
}
