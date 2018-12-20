extern crate utils;

use std::collections::VecDeque;
use utils::Day;

pub struct Day20 {
    regex: String,
}

impl Day20 {
    pub fn new(input: String) -> Day20 {
        let regex = input.trim().to_string();
        Day20 { regex }
    }

    fn create_row(&self, map: &mut VecDeque<VecDeque<char>>, on_top: bool) {
        let mut row: VecDeque<char> = VecDeque::new();
        for _ in 0..map[0].len() {
            row.push_back('#');
        }
        if on_top {
            map.push_front(row.clone());
            map.push_front(row);
        } else {
            map.push_back(row.clone());
            map.push_back(row.clone());
        }
    }

    fn create_column(&self, map: &mut VecDeque<VecDeque<char>>, in_front: bool) {
        for i in 0..map.len() {
            if in_front {
                map[i].push_front('#');
                map[i].push_front('#');
            } else {
                map[i].push_back('#');
                map[i].push_back('#');
            }
        }
    }

    fn make_move(
        &self,
        map: &mut VecDeque<VecDeque<char>>,
        move_dir: char,
        x: &mut usize,
        y: &mut usize,
    ) {
        match move_dir {
            'N' => {
                if *y == 1 {
                    self.create_row(map, true);
                    map[*y][*x] = '.';
                    map[*y + 1][*x] = '-';
                } else {
                    map[*y - 1][*x] = '-';
                    map[*y - 2][*x] = '.';
                    *y -= 2;
                }
            }
            'E' => {
                if *x == map[*y].len() - 2 {
                    self.create_column(map, false);
                }
                map[*y][*x + 1] = '|';
                map[*y][*x + 2] = '.';
                *x += 2;
            }
            'S' => {
                if *y == map.len() - 2 {
                    self.create_row(map, false);
                }
                map[*y + 1][*x] = '-';
                map[*y + 2][*x] = '.';
                *y += 2;
            }
            'W' => {
                if *x == 1 {
                    self.create_column(map, true);
                    map[*y][*x] = '.';
                    map[*y][*x + 1] = '|';
                } else {
                    map[*y][*x - 1] = '|';
                    map[*y][*x - 2] = '.';
                    *x -= 2;
                }
            }
            _ => {}
        }
    }

    fn build_map(&self) -> VecDeque<VecDeque<char>> {
        let mut map: VecDeque<VecDeque<char>> = VecDeque::new();
        let mut stack: Vec<char> = Vec::new();
        let (mut x, mut y) = (1, 1);
        for _ in 0..3 {
            let mut row: VecDeque<char> = VecDeque::new();
            for _ in 0..3 {
                row.push_back('#');
            }
            map.push_back(row);
        }
        map[y][x] = 'X';
        for c in self.regex.chars() {
            match c {
                '^' | '$' => {}
                '(' => stack.push(c),
                '|' => {
                    while *stack.last().unwrap() != '(' {
                        let move_dir = match stack.pop().unwrap() {
                            'N' => 'S',
                            'E' => 'W',
                            'S' => 'N',
                            'W' => 'E',
                            _ => '!',
                        };
                        if move_dir != '!' {
                            self.make_move(&mut map, move_dir, &mut x, &mut y);
                        }
                    }
                    stack.push(c);
                }
                ')' => {
                    if *stack.last().unwrap() == '|' {
                        while *stack.last().unwrap() != '(' {
                            stack.pop();
                        }
                    } else {
                        while *stack.last().unwrap() != '(' {
                            let move_dir = match stack.pop().unwrap() {
                                'N' => 'S',
                                'E' => 'W',
                                'S' => 'N',
                                'W' => 'E',
                                _ => '!',
                            };
                            self.make_move(&mut map, move_dir, &mut x, &mut y);
                        }
                    }
                    stack.pop();
                }
                _ => {
                    self.make_move(&mut map, c, &mut x, &mut y);
                    stack.push(c);
                }
            }
        }
        map
    }

    fn check_node(
        &self,
        map: &VecDeque<VecDeque<char>>,
        distances: &mut Vec<Vec<usize>>,
        x: usize,
        y: usize,
    ) {
        if map[y - 1][x] == '-' && distances[y - 2][x] > distances[y][x] + 1 {
            distances[y - 2][x] = distances[y][x] + 1;
            self.check_node(map, distances, x, y - 2);
        }
        if map[y + 1][x] == '-' && distances[y + 2][x] > distances[y][x] + 1 {
            distances[y + 2][x] = distances[y][x] + 1;
            self.check_node(map, distances, x, y + 2);
        }
        if map[y][x - 1] == '|' && distances[y][x - 2] > distances[y][x] + 1 {
            distances[y][x - 2] = distances[y][x] + 1;
            self.check_node(map, distances, x - 2, y);
        }
        if map[y][x + 1] == '|' && distances[y][x + 2] > distances[y][x] + 1 {
            distances[y][x + 2] = distances[y][x] + 1;
            self.check_node(map, distances, x + 2, y);
        }
    }

    fn traverse_map(&self, map: &VecDeque<VecDeque<char>>) -> Vec<Vec<usize>> {
        let (mut x, mut y) = (0, 0);
        let mut distances: Vec<Vec<usize>> = Vec::new();
        for (i, row) in map.iter().enumerate() {
            let mut dist_row: Vec<usize> = Vec::new();
            for (j, val) in row.iter().enumerate() {
                dist_row.push(std::usize::MAX);
                if *val == 'X' {
                    x = j;
                    y = i;
                }
            }
            distances.push(dist_row);
        }
        distances[y][x] = 0;
        self.check_node(&map, &mut distances, x, y);
        distances
            .iter()
            .map(|row| row.iter().map(|val| *val + 1).collect()) // <- THIS IS IMPORRTANT, DON'T TO SUBTRACT THAT
            .collect()
    }
}

impl Day for Day20 {
    fn get_part_a_result(&self) -> String {
        let map: VecDeque<VecDeque<char>> = self.build_map();
        let distances = self.traverse_map(&map);
        let max = distances
            .into_iter()
            .map(|row| row.into_iter().max().unwrap())
            .max()
            .unwrap()
            - 1;
        max.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let map: VecDeque<VecDeque<char>> = self.build_map();
        let distances = self.traverse_map(&map);
        let rooms = distances
            .into_iter()
            .map(|row| row.into_iter().filter(|val| *val > 1000).count())
            .sum::<usize>();
        rooms.to_string()
    }
}
