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
            // well this ended up being overcomplicated lol
            let count: i16 = counts
                .iter()
                .map(|entry| *entry.1)
                .filter(|val| *val == 2 || *val == 3)
                .fold(1, |acc, val| acc * val);
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
        let mut ids: HashMap<(&str, &str), usize> = HashMap::new();
        let mut min_diff = input[0].len();
        let mut string1_iter = input.iter().cloned();
        for i in 0..input.len() {
            let mut string2_iter = input.iter().skip(i + 1).cloned();
            let string1 = string1_iter.next().unwrap();
            let string1_chars: Vec<char> = string1.chars().collect();
            for string2 in string2_iter {
                let mut diffs = 0;
                for (j, c) in string2.chars().enumerate() {
                    if c != string1_chars[j] {
                        diffs += 1;
                    }
                }
                min_diff = if diffs < min_diff { diffs } else { min_diff };
                let tuple = (string1, string2);
                ids.insert(tuple, diffs);
            }
        }
        let mut result = String::new();
        for (key, val) in ids {
            if val == min_diff {
                let mut chars1 = key.0.chars();
                let mut chars2 = key.1.chars();
                let mut k = 0;
                let len = key.0.chars().count();
                while k < len {
                    let c1 = chars1.next();
                    let c2 = chars2.next();
                    if c1 == c2 {
                        result.push(c1.unwrap_or_default());
                    }
                    k += 1;
                }
                break;
            }
        }
        result
    }
}
