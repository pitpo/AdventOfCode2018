extern crate utils;

use utils::Day;

pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new(input: String) -> Day6 {
        Day6 { input }
    }

    fn parse_input(&self) -> Vec<Vec<usize>> {
        utils::extract_unsigned_integers_from_string(&self.input)
    }

    fn get_distance(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        let first = if x1 < x2 {x2 - x1} else {x1 - x2};
        let second = if y1 < y2 {y2 - y1} else {y1 - y2};
        first + second
    }
}

impl Day for Day6 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let mut vec: Vec<(Vec<(usize, usize)>)> = Vec::new();
        let max_x: usize = *input.iter().max_by_key(|x| x[0]).unwrap().get(0).unwrap();
        let max_y: usize = *input.iter().max_by_key(|x| x[1]).unwrap().get(1).unwrap();
        for i in 0..(max_y+1) {
            vec.push(Vec::new());
            for _ in 0..(max_x+1) {
                vec[i].push((0,0));
            }
        }
        for i in 0..input.len() {
            let mut x = input[i][0];
            let mut y = input[i][1];
            vec[input[i][1]][input[i][0]] = (i+1, 0);
            for j in 0..(max_y+1) {
                for k in 0..(max_x+1) {
                    let dist = self.get_distance((x, y), (k,j));
                    if vec[j][k].0 == 0 || dist < vec[j][k].1 {
                        vec[j][k].0 = i+1;
                        vec[j][k].1 = dist;
                    } else if vec[j][k].0 != 0 && vec[j][k].0 != i+1 && dist == vec[j][k].1 {
                        vec[j][k].0 = std::usize::MAX;
                    }
                }
            }
        }
        let mut max_area = 0;
        'outer: for i in 0..input.len() {
            let mut area = 0;
            for j in 0..(max_y+1) {
                for k in 0..(max_x+1) {
                    if vec[j][k].0 == i+1 {
                        if j == 0 || j == max_y || k == 0 || k == max_x {
                            continue 'outer;
                        }
                        area += 1;
                    }
                }
            }
            if area > max_area {
                max_area = area;
            }
        }
        max_area.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let mut vec: Vec<(Vec<usize>)> = Vec::new();
        let max_x: usize = *input.iter().max_by_key(|x| x[0]).unwrap().get(0).unwrap();
        let max_y: usize = *input.iter().max_by_key(|x| x[1]).unwrap().get(1).unwrap();
        for i in 0..(max_y+1) {
            vec.push(Vec::new());
            for j in 0..(max_x+1) {
                let dist = input.iter().fold(0, |acc, x| acc + self.get_distance((x[0], x[1]), (j, i)));
                vec[i].push(dist);
            }
        }
        let mut result = 0;
        for i in 0..(max_y+1) {
            for j in 0..(max_x+1) {
                if vec[i][j] < 10000 {
                    result += 1;
                }
            }
        }
        result.to_string()
    }
}
