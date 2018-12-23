extern crate utils;

use utils::Day;

pub struct Day3 {
    input: Vec<Vec<usize>>,
}

impl Day3 {
    pub fn new(input: String) -> Day3 {
        let input = utils::extract_unsigned_integers_from_string(&input);
        Day3 { input }
    }
}

impl Day for Day3 {
    fn get_part_a_result(&self) -> String {
        let mut array: Vec<Vec<usize>> = Vec::new();
        for claim in self.input.iter() {
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
        let result = array.into_iter().fold(0, |acc, row| {
            acc + row.into_iter().filter(|&num| num > 1).count()
        });
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        for (i, claim) in self.input.iter().enumerate() {
            let mut overlaps = false;
            for (j, other_claim) in self.input.iter().enumerate() {
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
