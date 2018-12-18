extern crate utils;

use utils::Day;

pub struct Day18 {
    initial_map: Vec<Vec<char>>,
}

impl Day18 {
    pub fn new(input: String) -> Day18 {
        let initial_map = utils::get_char_arrays(&input);
        Day18 { initial_map }
    }

    fn get_info_about_node(
        &self,
        node: char,
        open_acres: &mut usize,
        tree_acres: &mut usize,
        lumberyard_acres: &mut usize,
    ) {
        match node {
            '.' => *open_acres += 1,
            '|' => *tree_acres += 1,
            '#' => *lumberyard_acres += 1,
            _ => {}
        }
    }

    fn get_updated_node(&self, x: usize, y: usize, map: &Vec<Vec<char>>) -> char {
        let (mut open_acres, mut tree_acres, mut lumberyard_acres) = (0, 0, 0);
        if y > 0 {
            self.get_info_about_node(
                map[y - 1][x],
                &mut open_acres,
                &mut tree_acres,
                &mut lumberyard_acres,
            );
            if x > 0 {
                self.get_info_about_node(
                    map[y - 1][x - 1],
                    &mut open_acres,
                    &mut tree_acres,
                    &mut lumberyard_acres,
                );
            }
            if x + 1 < map[y - 1].len() {
                self.get_info_about_node(
                    map[y - 1][x + 1],
                    &mut open_acres,
                    &mut tree_acres,
                    &mut lumberyard_acres,
                );
            }
        }
        if x > 0 {
            self.get_info_about_node(
                map[y][x - 1],
                &mut open_acres,
                &mut tree_acres,
                &mut lumberyard_acres,
            );
        }
        if x + 1 < map[y].len() {
            self.get_info_about_node(
                map[y][x + 1],
                &mut open_acres,
                &mut tree_acres,
                &mut lumberyard_acres,
            );
        }
        if y + 1 < map.len() {
            self.get_info_about_node(
                map[y + 1][x],
                &mut open_acres,
                &mut tree_acres,
                &mut lumberyard_acres,
            );
            if x > 0 {
                self.get_info_about_node(
                    map[y + 1][x - 1],
                    &mut open_acres,
                    &mut tree_acres,
                    &mut lumberyard_acres,
                );
            }
            if x + 1 < map[y + 1].len() {
                self.get_info_about_node(
                    map[y + 1][x + 1],
                    &mut open_acres,
                    &mut tree_acres,
                    &mut lumberyard_acres,
                );
            }
        }
        match map[y][x] {
            '.' => if tree_acres >= 3 {
                return '|';
            } else {
                return '.';
            },
            '|' => if lumberyard_acres >= 3 {
                return '#';
            } else {
                return '|';
            },
            '#' => if lumberyard_acres >= 1 && tree_acres >= 1 {
                return '#';
            } else {
                return '.';
            },
            _ => return map[y][x],
        }
    }
}

impl Day for Day18 {
    fn get_part_a_result(&self) -> String {
        let mut map = self.initial_map.clone();
        for _ in 0..10 {
            let mut updated_map = map.clone();
            for (i, row) in updated_map.iter_mut().enumerate() {
                for (j, node) in row.iter_mut().enumerate() {
                    *node = self.get_updated_node(j, i, &map);
                }
            }
            map = updated_map;
        }
        let (mut tree_acres, mut lumberyard_acres) = (0, 0);
        for row in map {
            for node in row {
                if node == '|' {
                    tree_acres += 1;
                }
                if node == '#' {
                    lumberyard_acres += 1;
                }
            }
        }
        (tree_acres * lumberyard_acres).to_string()
    }
    fn get_part_b_result(&self) -> String {
        let mut map = self.initial_map.clone();
        let mut arr: Vec<usize> = Vec::new();
        let mut first_appearance = 0;
        let mut cycle: Vec<usize> = Vec::new();
        loop {
            let mut updated_map = map.clone();
            for (i, row) in updated_map.iter_mut().enumerate() {
                for (j, node) in row.iter_mut().enumerate() {
                    *node = self.get_updated_node(j, i, &map);
                }
            }
            map = updated_map;
            let (mut tree_acres, mut lumberyard_acres) = (0, 0);
            for row in &map {
                for node in row {
                    if *node == '|' {
                        tree_acres += 1;
                    }
                    if *node == '#' {
                        lumberyard_acres += 1;
                    }
                }
            }
            let result = tree_acres * lumberyard_acres;
            if arr.iter().filter(|&&num| num == result).count() == 2 {
                if first_appearance == 0 {
                    first_appearance = arr.len();
                }
            }
            if arr.iter().filter(|&&num| num == result).count() == 4 {
                break;
            }
            if first_appearance != 0 && !cycle.contains(&result) {
                cycle.push(result);
            }
            arr.push(result);
        }
        cycle[(1000000000-first_appearance-1)%cycle.len()].to_string()
    }
}
