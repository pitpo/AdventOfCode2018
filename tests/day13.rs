extern crate day13;
extern crate utils;

use utils::Day;

#[test]
fn day13_a() {
    let solver = day13::Day13::new(String::from("/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   "));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "7,3");
}

#[test]
fn day13_b() {
    let solver = day13::Day13::new(String::from("/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/"));

    let ans = solver.get_part_b_result();
    
    assert_eq!(&ans, "6,4");
}