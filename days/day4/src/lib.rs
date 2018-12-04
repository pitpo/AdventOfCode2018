extern crate utils;

use std::collections::HashMap;
use utils::Day;

pub struct Day4 {
    input: String,
}

impl Day4 {
    pub fn new(input: String) -> Day4 {
        Day4 { input }
    }

    fn parse_input(&self) -> Vec<Vec<usize>> {
        self.input
            .lines()
            .map(|line| {
                line.split(|c| c == '[' || c == ']' || c == ' ' || c == '-' || c == ':')
                    .map(|s: &str| s.chars().filter(|c| c.is_digit(10)).collect::<String>())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap_or_default())
                    .collect()
            }).collect()
    }

    fn get_time_heatmap(&self, mut input: Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> {
        let mut heatmap: HashMap<usize, Vec<usize>> = HashMap::new();
        input.sort_by(|a, b| {
            for i in 0..5 {
                if a[i] != b[i] {
                    return a[i].cmp(&b[i]);
                }
            }
            std::cmp::Ordering::Equal
        });
        let mut last_id = 0;
        let mut i = 0;
        loop {
            if i == input.len() {
                break;
            }
            if input[i].len() == 6 {
                last_id = input[i][5];
                i += 1;
            }
            let entry = heatmap.entry(last_id).or_insert_with(|| {
                let mut v: Vec<usize> = Vec::new();
                for i in 0..60 {
                    v.push(0);
                }
                v
            });
            for j in 0..60 {
                (*entry)[j] += if j >= input[i][4] && j < input[i + 1][4] {
                    1
                } else {
                    0
                };
            }
            i += 2;
        }
        heatmap
    }
}

// very ugly, don't have time to fix it now
impl Day for Day4 {
    fn get_part_a_result(&self) -> String {
        let mut input = self.parse_input();
        let heatmap = self.get_time_heatmap(input);
        let mut max_minutes = 0;
        let mut max_id = 0;
        let mut max_freq = 0;
        let mut max_freq_minute = 0;
        for (id, sched) in heatmap {
            let mut minutes = 0;
            let mut local_max_freq = 0;
            let mut local_max_freq_minute = 0;
            for (i, freq) in sched.into_iter().enumerate() {
                minutes += freq;
                if freq > local_max_freq {
                    local_max_freq = freq;
                    local_max_freq_minute = i;
                }
            }
            if minutes > max_minutes || (minutes == max_minutes && local_max_freq > max_freq) {
                max_minutes = minutes;
                max_id = id;
                max_freq = local_max_freq;
                max_freq_minute = local_max_freq_minute;
            }
        }
        let result = max_id * max_freq_minute;
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let mut input = self.parse_input();
        let heatmap = self.get_time_heatmap(input);
        let mut max_id = 0;
        let mut max_freq = 0;
        let mut max_freq_minute = 0;
        for (id, sched) in heatmap {
            for (i, freq) in sched.into_iter().enumerate() {
                if freq > max_freq {
                    max_freq = freq;
                    max_freq_minute = i;
                    max_id = id;
                }
            }
        }
        let result = max_id * max_freq_minute;
        result.to_string()
    }
}
