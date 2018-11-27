extern crate day1;
extern crate utils;

use std::env;
use utils::Day;

fn main() {
    let solver: Box<Day>;
    let env_arg = env::args().nth(1).unwrap_or_default();
    match env_arg.as_ref() {
        "day1" => solver = Box::new(day1::Day1::new()),
        _ => panic!("Unknown or missing argument"),
    }
    println!("-----------PART A SOLUTION-----------");
    println!("{}", solver.get_part_a_result());
    println!("-----------PART B SOLUTION-----------");
    println!("{}", solver.get_part_b_result());
}
