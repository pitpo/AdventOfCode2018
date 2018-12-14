extern crate day1;
extern crate day2;
extern crate day3;
extern crate day4;
extern crate day5;
extern crate day6;
extern crate day7;
extern crate day8;
extern crate day9;
extern crate day10;
extern crate day11;
extern crate day12;
extern crate day13;
extern crate day14;
extern crate day15;
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
        "day4" => solver = Box::new(day4::Day4::new(get_input_for_day(4))),
        "day5" => solver = Box::new(day5::Day5::new(get_input_for_day(5))),
        "day6" => solver = Box::new(day6::Day6::new(get_input_for_day(6))),
        "day7" => solver = Box::new(day7::Day7::new(get_input_for_day(7))),
        "day8" => solver = Box::new(day8::Day8::new(get_input_for_day(8))),
        "day9" => solver = Box::new(day9::Day9::new(get_input_for_day(9))),
        "day10" => solver = Box::new(day10::Day10::new(get_input_for_day(10))),
        "day11" => solver = Box::new(day11::Day11::new(get_input_for_day(11))),
        "day12" => solver = Box::new(day12::Day12::new(get_input_for_day(12))),
        "day13" => solver = Box::new(day13::Day13::new(get_input_for_day(13))),
        "day14" => solver = Box::new(day14::Day14::new(get_input_for_day(14))),
        "day15" => solver = Box::new(day15::Day15::new(get_input_for_day(15))),
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
