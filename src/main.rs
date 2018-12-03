extern crate day1;
extern crate day2;
extern crate day3;
extern crate utils;

use std::env;
use std::time::SystemTime;
use utils::network::*;
use utils::Day;

fn main() {
    let solver: Box<Day>;
    let env_arg = env::args().nth(1).unwrap_or_default();
    match env_arg.as_ref() {
        "day1" => solver = Box::new(day1::Day1::new(get_input_for_day(1))),
        "day2" => solver = Box::new(day2::Day2::new(get_input_for_day(2))),
        "day3" => solver = Box::new(day3::Day3::new(get_input_for_day(3))),
        _ => panic!("Unknown or missing argument"),
    }
    println!("-----------PART A SOLUTION-----------");
    let timer = SystemTime::now();
    println!("{}", solver.get_part_a_result());
    let duration = SystemTime::now().duration_since(timer).unwrap();
    println!(
        "Took {}.{:09}s",
        duration.as_secs(),
        duration.subsec_nanos()
    );
    println!("-----------PART B SOLUTION-----------");
    let timer = SystemTime::now();
    println!("{}", solver.get_part_b_result());
    let duration = SystemTime::now().duration_since(timer).unwrap();
    println!(
        "Took {}.{:09}s",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}
