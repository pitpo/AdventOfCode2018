extern crate utils;

use std::collections::VecDeque;
use utils::Day;

pub struct Day12 {
    initial_state: Vec<char>,
    rules: Vec<(Vec<char>, char)>,
}

impl Day12 {
    pub fn new(input: String) -> Day12 {
        let input: Vec<String> = input
            .split(|c| c == ':' || c == '\n')
            .map(|l| l.chars().collect())
            .collect();
        let initial_state: Vec<char> = input[1].trim().chars().collect();
        let rules: Vec<(Vec<char>, char)> = input
            .into_iter()
            .skip(3)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let tmp = s
                    .split('=')
                    .map(|l| l.trim().chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>();
                (
                    tmp.get(0).unwrap().clone(),
                    tmp.get(1).unwrap().iter().last().unwrap().clone(),
                )
            }).collect();
        Day12 {
            initial_state,
            rules,
        }
    }
}

impl Day for Day12 {
    fn get_part_a_result(&self) -> String {
        let mut arr: VecDeque<char> = VecDeque::from(self.initial_state.clone());
        let mut last_index: isize = (arr.len() - 1) as isize;
        let mut first_index: isize = 0;
        let mut array_shift: isize = -4;
        for _ in 0..4 {
            arr.push_front('.');
            arr.push_back('.');
        }
        for _ in 0..20 {
            let mut arr2: VecDeque<char> = VecDeque::with_capacity(arr.len());
            for _ in 0..arr.len() {
                arr2.push_back('.');
            }
            for i in 0..arr.len() - 5 {
                for rule in &self.rules {
                    let mut matched = true;
                    for j in 0..5 {
                        if arr[i + j] != rule.0[j] {
                            matched = false;
                        }
                    }
                    if matched {
                        arr2[i + 2] = rule.1;
                        break;
                    }
                }
            }
            for i in 0..arr.len() {
                if (i as isize) < first_index - array_shift && arr2[i] == '#' {
                    let diff = 4 - (i as isize);
                    array_shift -= diff;
                    first_index -= diff;
                    if diff > 0 {
                        for _ in 0..diff {
                            arr2.push_front('.');
                        }
                    }
                }
                if i > arr2.len() - 5 && arr2[i] == '#' {
                    let diff = (i as isize) - last_index + array_shift;
                    last_index += diff;
                    for _ in 0..diff {
                        arr2.push_back('.');
                    }
                }
            }
            arr = arr2;
        }
        let result: isize = arr
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                if c == '#' {
                    return (i as isize) + array_shift;
                }
                0
            }).sum();
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let mut arr: VecDeque<char> = VecDeque::from(self.initial_state.clone());
        let mut last_index: isize = (arr.len() - 1) as isize;
        let mut first_index: isize = 0;
        let mut array_shift: isize = -4;
        let mut prev_result = 0;
        let mut prev_diff = 0;
        for _ in 0..4 {
            arr.push_front('.');
            arr.push_back('.');
        }
        for k in 0..1000000 {
            let mut arr2: VecDeque<char> = VecDeque::with_capacity(arr.len());
            for _ in 0..arr.len() {
                arr2.push_back('.');
            }
            for i in 0..arr.len() - 5 {
                for rule in &self.rules {
                    let mut matched = true;
                    for j in 0..5 {
                        if arr[i + j] != rule.0[j] {
                            matched = false;
                        }
                    }
                    if matched {
                        arr2[i + 2] = rule.1;
                        break;
                    }
                }
            }
            for i in 0..arr.len() {
                if (i as isize) < first_index - array_shift && arr2[i] == '#' {
                    let diff = 4 - (i as isize);
                    array_shift -= diff;
                    first_index -= diff;
                    if diff > 0 {
                        for _ in 0..diff {
                            arr2.push_front('.');
                        }
                    }
                }
                if i > arr2.len() - 5 && arr2[i] == '#' {
                    let diff = (i as isize) - last_index + array_shift;
                    last_index += diff;
                    for _ in 0..diff {
                        arr2.push_back('.');
                    }
                }
            }
            arr = arr2;
            let result: isize = arr
                .iter()
                .enumerate()
                .map(|(i, &c)| {
                    if c == '#' {
                        return (i as isize) + array_shift;
                    }
                    0
                }).sum();
            let diff = result - prev_result;
            if diff == prev_diff {
                return ((result as usize) + ((50000000000 as usize) - k - 1) * (diff as usize)).to_string();
            }
            prev_result = result;
            prev_diff = diff;
        }
        String::new()
    }
}
