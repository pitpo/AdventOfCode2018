extern crate utils;

use utils::Day;

pub struct Day3 {
    input: String,
}

impl Day3 {
    pub fn new(input: String) -> Day3 {
        Day3 { input }
    }

    fn parse_input(&self) -> Vec<Vec<usize>> {
        utils::extract_unsigned_integers_from_string(&self.input)
    }
}

impl Day for Day3 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let mut array: Vec<Vec<usize>> = Vec::new();
        let mut result = 0;
        for claim in input {
            for i in 0..(claim[2] + claim[4]) {
                if i >= array.len() && claim[2] + claim[4] >= array.len() {
                    array.push(Vec::new())
                }
                if i >= claim[2] {
                    for j in 0..(claim[1] + claim[3]) {
                        if j >= array[i].len() && claim[1] + claim[3] >= array[i].len() {
                            array[i].push(0);
                        }
                    }
                    for j in claim[1]..(claim[1] + claim[3]) {
                        array[i][j] += 1;
                    }
                }
            }
        }
        for row in array {
            for i in row {
                if i > 1 {
                    result += 1;
                }
            }
        }
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        for (i, claim) in input.iter().enumerate() {
            let mut overlaps = false;
            for (j, other_claim) in input.iter().enumerate() {
                if i != j {
                    let other_x_end = other_claim[1] + other_claim[3] - 1;
                    let other_y_end = other_claim[2] + other_claim[4] - 1;
                    let x_end = claim[1] + claim[3] - 1;
                    let y_end = claim[2] + claim[4] - 1;
                    let overlaps_in_x = x_end >= other_claim[1] && other_x_end >= claim[1];
                    let overlaps_in_y = y_end >= other_claim[2] && other_y_end >= claim[2];
                    if overlaps_in_x && overlaps_in_y {
                        overlaps = true;
                        break;
                    }
                }
            }
            if !overlaps {
                return claim[0].to_string();
            }
        }
        String::new()
    }
}
