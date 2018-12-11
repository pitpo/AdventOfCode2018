extern crate utils;

use utils::Day;

pub struct Day11 {
    input: u16,
}

impl Day11 {
    pub fn new(input: String) -> Day11 {
        let input = input.trim().parse().unwrap();
        Day11 { input }
    }

    fn calculate_power_level(&self, x: usize, y: usize) -> i16 {
        let rack_id = x + 10;
        let mut power_level = rack_id * y + self.input as usize;
        power_level = (power_level * rack_id) / 100;
        let digit: i16 = power_level
            .to_string()
            .chars()
            .last()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        return digit - 5;
    }

    fn sum_k(&self, arr: &Vec<Vec<i16>>, x: usize, y: usize, k: usize) -> i16 {
        let mut sum = 0;
        for i in y..y + k {
            for j in x..x + k {
                sum += arr[i][j];
            }
        }
        sum
    }

    fn get_power_cell_array(&self) -> Vec<Vec<i16>> {
        let mut vec: Vec<Vec<i16>> = Vec::with_capacity(300);
        for i in 0..300 {
            vec.push(Vec::with_capacity(300));
            for j in 0..300 {
                vec[i].push(self.calculate_power_level(j, i));
            }
        }
        vec
    }
}

impl Day for Day11 {
    fn get_part_a_result(&self) -> String {
        let vec = self.get_power_cell_array();
        let (mut max_sum, mut max_region_x, mut max_region_y) = (0, 0, 0);
        for i in 0..298 {
            for j in 0..298 {
                let sum = self.sum_k(&vec, j, i, 3);
                if sum > max_sum {
                    max_sum = sum;
                    max_region_x = j;
                    max_region_y = i;
                }
            }
        }
        String::from(format!("{},{}", max_region_x, max_region_y))
    }
    fn get_part_b_result(&self) -> String {
        let vec = self.get_power_cell_array();
        let (mut max_sum, mut max_region_x, mut max_region_y, mut max_region_size) = (0, 0, 0, 0);
        for k in 3..300 {
            let mut max_region_sum = 0;
            for i in 0..(300 - k + 1) {
                for j in 0..(300 - k + 1) {
                    let sum = self.sum_k(&vec, j, i, k);
                    if sum > max_region_sum {
                        max_region_sum = sum;
                    }
                    if sum > max_sum {
                        max_sum = sum;
                        max_region_x = j;
                        max_region_y = i;
                        max_region_size = k;
                    }
                }
            }
            if max_region_sum + (k as i16) < max_sum {
                break;
            }
        }
        String::from(format!(
            "{},{},{}",
            max_region_x, max_region_y, max_region_size
        ))
    }
}
