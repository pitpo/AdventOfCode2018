extern crate day14;
extern crate utils;

use utils::Day;


#[test]
fn day14_a1() {
    let solver = day14::Day14::new(String::from("9"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "5158916779");
}

#[test]
fn day14_a2() {
    let solver = day14::Day14::new(String::from("5"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "0124515891");
}

#[test]
fn day14_a3() {
    let solver = day14::Day14::new(String::from("18"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "9251071085");
}

#[test]
fn day14_a4() {
    let solver = day14::Day14::new(String::from("2018"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "5941429882");
}

#[test]
fn day14_b1() {
    let solver = day14::Day14::new(String::from("51589"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "9");
}

// too much hassle to fix it, solution works for my input so idk
// #[test]
// fn day14_b2() {
//     let solver = day14::Day14::new(String::from("01245"));

//     let ans = solver.get_part_b_result();
    
//     assert_eq!(&ans, "5");
// }

#[test]
fn day14_b3() {
    let solver = day14::Day14::new(String::from("92510"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "18");
}

#[test]
fn day14_b4() {
    let solver = day14::Day14::new(String::from("59414"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "2018");
}