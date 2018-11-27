extern crate utils;

use utils::Day;

pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        Day1 { input }
    }
}

impl Day for Day1 {
    fn get_part_a_result(&self) -> String {
        self.input.clone()
    }
    fn get_part_b_result(&self) -> String {
        self.input.clone()
    }
}
