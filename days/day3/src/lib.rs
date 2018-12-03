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
        self.input
            .lines()
            .map(|line: &str| {
                line.split(|c: char| c == '@' || c == ',' || c == ':' || c == 'x')
                    .map(|s: &str| s.chars().filter(|c| c.is_digit(10)).collect::<String>())
                    .map(|s: String| s.parse().unwrap())
                    .collect::<Vec<usize>>()
            }).collect()
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
        let mut input = self.parse_input();
        let mut array: Vec<Vec<usize>> = Vec::new();
        let mut result = 0;
        // Due to borrowing issues, it has to look this way... for now at least
        for x in 0..input.len() {
            for i in 0..(input[x][2] + input[x][4]) {
                if i >= array.len() && input[x][2] + input[x][4] >= array.len() {
                    array.push(Vec::new())
                }
                if i >= input[x][2] {
                    for j in 0..(input[x][1] + input[x][3]) {
                        if j >= array[i].len() && input[x][1] + input[x][3] >= array[i].len() {
                            array[i].push(0);
                        }
                    }
                    for j in input[x][1]..(input[x][1] + input[x][3]) {
                        if array[i][j] != 0 {
                            input[x].push(array[i][j]);
                            for k in 0..input.len() {
                                if input[k][0] == array[i][j] {
                                    let id = input[x][0];
                                    input[k].push(id);
                                }
                            }
                        }
                        array[i][j] = input[x][0];
                    }
                }
            }
        }
        for claim in input {
            if claim.len() == 5 {
                result = claim[0];
                break;
            }
        }
        result.to_string()
    }
}
