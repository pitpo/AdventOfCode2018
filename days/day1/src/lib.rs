extern crate utils;

use std::collections::HashSet;
use utils::Day;

pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        Day1 { input }
    }

    fn parse_input(&self) -> Vec<i64> {
        self.input
            .split(|c| c == '\n' || c == '+')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl Day for Day1 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let mut acc: i64 = 0;
        for operation in input {
            acc += operation;
        }
        String::from(acc.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let mut set: HashSet<i64> = HashSet::new();
        let mut acc: i64 = 0;
        let mut i = 0;
        loop {
            if set.contains(&acc) {
                break;
            }
            set.insert(acc);
            acc += input[i];
            i = if i == input.len() - 1 { 0 } else { i + 1 };
        }
        String::from(acc.to_string())
    }
}
