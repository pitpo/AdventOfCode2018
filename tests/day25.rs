extern crate day25;
extern crate utils;

use utils::Day;

#[test]
fn day25_a1() {
    let solver = day25::Day25::new(String::from("0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "2");
}

#[test]
fn day25_a2() {
    let solver = day25::Day25::new(String::from("-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "4");
}

#[test]
fn day25_a3() {
    let solver = day25::Day25::new(String::from("1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "3");
}

#[test]
fn day25_a4() {
    let solver = day25::Day25::new(String::from("1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "8");
}