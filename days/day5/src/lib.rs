extern crate utils;

use utils::Day;

pub struct Day5 {
    input: String,
}

impl Day5 {
    pub fn new(input: String) -> Day5 {
        Day5 { input }
    }

    fn parse_input(&self) -> Vec<char> {
        self.input.chars().filter(|c| !c.is_whitespace()).collect()
    }

    fn remove_and_count_units(&self, input: &Vec<char>, char_to_remove: Option<char>) -> usize {
        let mut chars: Vec<char> = input.clone();
        if let Some(c) = char_to_remove {
            chars = input
                .iter()
                .cloned()
                .filter(|&x| x.to_ascii_lowercase() != c)
                .collect();
        }
        let (mut i, mut j, null) = (0, 1, 0 as char);
        while i < chars.len() {
            if j < chars.len()
                && chars[i] != chars[j]
                && chars[i].to_ascii_lowercase() == chars[j].to_ascii_lowercase()
            {
                chars[i] = null;
                chars[j] = null;
                while i > 0 && chars[i] == null {
                    i = i - 1;
                }
                while i < chars.len() && chars[i] == null {
                    i = i + 1;
                }
                while j < chars.len() - 1 && chars[j] == null {
                    j = j + 1;
                }
                while j > 0 && chars[j] == null {
                    j = j - 1;
                }
            } else {
                i = j;
                j = j + 1;
            }
        }
        chars.into_iter().filter(|&c| c != null).count()
    }
}

impl Day for Day5 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        self.remove_and_count_units(&input, None).to_string()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let mut min_length = input.len();
        for i in ('a' as u8)..('z' as u8) {
            let c = i as char;
            if input.contains(&c) {
                let local_len = self.remove_and_count_units(&input, Some(c));
                if local_len < min_length {
                    min_length = local_len;
                }
            }
        }
        min_length.to_string()
    }
}
