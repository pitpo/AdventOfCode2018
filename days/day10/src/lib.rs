extern crate utils;

use utils::Day;

pub struct Day10 {
    input: String,
}

impl Day10 {
    pub fn new(input: String) -> Day10 {
        Day10 { input }
    }

    fn parse_input(&self) -> Vec<Vec<isize>> {
        utils::extract_integers_from_string(&self.input)
    }
}

impl Day for Day10 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let (
            mut mini,
            mut min_boundary_size,
            mut min_minx,
            mut min_miny,
            mut min_xdiff,
            mut min_ydiff,
        ) = (0, std::isize::MAX, 0, 0, 0, 0);
        for i in 0..12000 {
            let minx = input.iter().map(|v| v[0] + i * v[2]).min().unwrap();
            let miny = input.iter().map(|v| v[1] + i * v[3]).min().unwrap();
            let maxx = input.iter().map(|v| v[0] + i * v[2]).max().unwrap();
            let maxy = input.iter().map(|v| v[1] + i * v[3]).max().unwrap();
            let boundary_size = maxx - minx + maxy - miny;
            if boundary_size < min_boundary_size {
                min_boundary_size = boundary_size;
                mini = i;
                min_minx = minx;
                min_miny = miny;
                min_xdiff = maxx - minx + 1;
                min_ydiff = maxy - miny + 1;
            }
        }
        let mut result: Vec<Vec<char>> = Vec::new();
        for _ in 0..min_ydiff {
            let mut vec: Vec<char> = Vec::new();
            for _ in 0..min_xdiff {
                vec.push('.');
            }
            result.push(vec);
        }
        for v in input {
            result[(v[1] + mini * v[3] - min_miny) as usize]
                [(v[0] + mini * v[2] - min_minx) as usize] = '#';
        }
        let mut result_str = String::with_capacity((min_ydiff * min_ydiff) as usize + result.len());
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result_str.push(result[i][j]);
            }
            result_str.push('\n');
        }
        result_str
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let (mut mini, mut min_boundary_size) = (0, std::isize::MAX);
        for i in 0..12000 {
            let minx = input.iter().map(|v| v[0] + i * v[2]).min().unwrap();
            let miny = input.iter().map(|v| v[1] + i * v[3]).min().unwrap();
            let maxx = input.iter().map(|v| v[0] + i * v[2]).max().unwrap();
            let maxy = input.iter().map(|v| v[1] + i * v[3]).max().unwrap();
            let boundary_size = maxx - minx + maxy - miny;
            if boundary_size < min_boundary_size {
                min_boundary_size = boundary_size;
                mini = i;
            }
        }
        mini.to_string()
    }
}
