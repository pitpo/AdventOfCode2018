extern crate utils;

use std::collections::HashMap;
use utils::Day;

pub struct Day2 {
    input: String,
}

impl Day2 {
    pub fn new(input: String) -> Day2 {
        Day2 { input }
    }

    fn parse_input(&self) -> Vec<&str> {
        self.input.lines().map(|s| s.trim()).collect()
    }
}

impl Day for Day2 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let mut twos: isize = 0;
        let mut threes: isize = 0;
        for line in input {
            let mut counts: HashMap<char, i16> = HashMap::new();
            for c in line.chars() {
                *counts.entry(c.into()).or_insert(0) += 1;
            }
            let count: i16 = counts
                .iter()
                .filter(|(_, &val)| val == 2 || val == 3)
                .fold(1, |acc, (_, &val)| acc * val);
            if count % 2 == 0 {
                twos += 1;
            }
            if count % 3 == 0 {
                threes += 1;
            }
        }
        let result = twos * threes;
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let ((s1, s2), _) = input
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(i, s1)| {
                input
                    .iter()
                    .skip(i + 1)
                    .map(|s2| {
                        let diff: usize = s1
                            .chars()
                            .zip(s2.chars())
                            .filter(|(c1, c2)| c1 != c2)
                            .count();
                        ((s1.clone(), s2.clone()), diff)
                    }).min_by_key(|tuple| tuple.1)
                    .ok_or(())
                    .ok()
            }).min_by_key(|tuple| tuple.1)
            .unwrap();
        let result = s1
            .chars()
            .zip(s2.chars())
            .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
            .collect::<String>();
        result
    }
}
