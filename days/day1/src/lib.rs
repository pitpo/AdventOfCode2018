extern crate utils;

use std::collections::HashSet;
use utils::Day;

pub struct Day1 {
    input: Vec<i32>,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        let input = input.split(|c| c == '\n' || c == '+')
                         .filter_map(|s| s.trim().parse().ok())
                         .collect();
        Day1 { input }
    }
}

impl Day for Day1 {
    fn get_part_a_result(&self) -> String {
        let acc: i32 = self.input.iter().sum();
        String::from(acc.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut set: HashSet<i32> = HashSet::new();
        let mut acc: i32 = 0;
        for num in self.input.iter().cycle() {
            if !set.insert(acc) {
                break;
            }
            acc += num;
        }
        acc.to_string()
    }
}
