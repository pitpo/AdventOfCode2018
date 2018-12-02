extern crate day2;
extern crate utils;

use utils::Day;

#[test]
fn day2_a() {
    let solver = day2::Day2::new(String::from("abcdef
    bababc
    abbcde
    abcccd
    aabcdd
    abcdee
    ababab"));
    
    let ans = solver.get_part_a_result();

    assert_eq!(&ans, "12");
}

#[test]
fn day2_b() {
    let solver = day2::Day2::new(String::from("abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"));
    
    let ans = solver.get_part_b_result();

    assert_eq!(&ans, "fgij");
}