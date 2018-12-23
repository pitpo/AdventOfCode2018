extern crate day23;
extern crate utils;

use utils::Day;

fn get_input() -> String {
    String::from("pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1")
}

#[test]
fn day23_a() {
    let solver = day23::Day23::new(String::from("pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "7");
}

#[test]
fn day23_b() {
    let solver = day23::Day23::new(String::from("pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "");
}