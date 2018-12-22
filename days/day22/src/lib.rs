extern crate utils;

use std::thread;
use utils::Day;

const X_MULTIPLIER: usize = 16807;
const Y_MULTIPLIER: usize = 48271;
const MODULO: usize = 20183;

pub struct Day22 {
    depth: usize,
    target_x: usize,
    target_y: usize,
}

struct Node {
    erosion_level: usize,
    node_type: usize,
    cost: usize,
}

impl Day22 {
    pub fn new(input: String) -> Day22 {
        let input = utils::extract_unsigned_integers_from_string(&input);
        let depth = input[0][0];
        let target_x = input[1][0];
        let target_y = input[1][1];
        Day22 {
            depth,
            target_x,
            target_y,
        }
    }

    fn build_map(&self) -> Vec<Vec<Node>> {
        let mut map: Vec<Vec<Node>> = Vec::new();
        for i in 0..self.target_y + 50 {
            let mut row: Vec<Node> = Vec::new();
            for j in 0..self.target_x + 50 {
                let mut erosion_level = 0;
                if (i == 0 && j == 0) || (i == self.target_y && j == self.target_x) {
                    erosion_level = self.depth % MODULO;
                } else if i > 0 && j > 0 {
                    erosion_level = ((row[j - 1].erosion_level * map[i - 1][j].erosion_level)
                        + self.depth)
                        % MODULO;
                } else if i > 0 {
                    erosion_level = ((i * Y_MULTIPLIER) + self.depth) % MODULO;
                } else if j > 0 {
                    erosion_level = ((j * X_MULTIPLIER) + self.depth) % MODULO;
                }
                let node_type = erosion_level % 3;
                let cost = std::usize::MAX;
                let node = Node {
                    erosion_level,
                    node_type,
                    cost,
                };
                row.push(node);
            }
            map.push(row);
        }
        map
    }

    fn calculate_costs(map: &mut Vec<Vec<Vec<Node>>>, x: usize, y: usize, equipment: usize) {
        let nodes = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for (next_x, next_y) in nodes {
            if next_x == std::usize::MAX
                || next_y == std::usize::MAX
                || next_x == map[0][0].len()
                || next_y == map[0].len()
            {
                continue;
            }
            if (equipment == 0 && map[0][next_y][next_x].node_type == 0)
                || (equipment == 1 && map[0][next_y][next_x].node_type == 2)
                || (equipment == 2 && map[0][next_y][next_x].node_type == 1)
            {
                continue;
            }
            if map[equipment][y][x].cost + 1 < map[equipment][next_y][next_x].cost {
                map[equipment][next_y][next_x].cost = map[equipment][y][x].cost + 1;
                Day22::calculate_costs(map, next_x, next_y, equipment);
            }
        }
        for i in 0..3 {
            if i == equipment
                || (i == 0 && map[0][y][x].node_type == 0)
                || (i == 1 && map[0][y][x].node_type == 2)
                || (i == 2 && map[0][y][x].node_type == 1)
            {
                continue;
            }
            if map[equipment][y][x].cost + 7 < map[i][y][x].cost {
                map[i][y][x].cost = map[equipment][y][x].cost + 7;
                Day22::calculate_costs(map, x, y, i);
            }
        }
    }
}

impl Day for Day22 {
    fn get_part_a_result(&self) -> String {
        let map = self.build_map();
        let mut sum = 0;
        for i in 0..=self.target_y {
            for j in 0..=self.target_x {
                sum += map[i][j].node_type;
            }
        }
        sum.to_string()
    }
    fn get_part_b_result(&self) -> String {
        println!("---This is gonna take a long while---");
        let mut map: Vec<Vec<Vec<Node>>> = Vec::new();
        for _ in 0..3 {
            map.push(self.build_map());
        }
        map[2][0][0].cost = 0;
        let builder = thread::Builder::new()
            .name("worker".to_string())
            .stack_size(1024 * 1024 * 64);
        let handler = builder
            .spawn(move || {
                Day22::calculate_costs(&mut map, 0, 0, 2);
                map
            }).unwrap();
        let map = handler.join().unwrap();
        map[2][self.target_y][self.target_x].cost.to_string()
    }
}
