extern crate utils;

use utils::Day;

pub struct Day8 {
    input: String,
    num_of_workers: usize,
    task_base_length: u8,
}

impl Day8 {
    pub fn new(input: String) -> Day8 {
        Day8 {
            input,
            num_of_workers: 5,
            task_base_length: 60,
        }
    }

    fn parse_input(&self) {
        
    }
}

impl Day for Day8 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        String::new()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        String::new()
    }
}
