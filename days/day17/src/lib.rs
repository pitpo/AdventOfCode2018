extern crate utils;

use utils::Day;

pub struct Day17 {
    input: Vec<Vec<usize>>,
    min_y: usize,
    max_y: usize,
    min_x: usize,
    max_x: usize,
}

impl Day17 {
    pub fn new(input: String) -> Day17 {
        let mut new_input = utils::extract_unsigned_integers_from_string(&input);
        for (i, line) in input.lines().enumerate() {
            if line.starts_with("x") {
                new_input[i].push(0);
            } else {
                new_input[i].push(1);
            }
        }
        let min_y = new_input
            .iter()
            .min_by_key(|v| if v[3] == 0 { v[1] } else { v[0] })
            .unwrap()
            .clone();
        let min_y = if min_y[3] == 0 { min_y[1] } else { min_y[0] };

        let max_y = new_input
            .iter()
            .max_by_key(|v| if v[3] == 0 { v[2] } else { v[0] })
            .unwrap()
            .clone();
        let max_y = if max_y[3] == 0 { max_y[2] } else { max_y[0] };

        let min_x = new_input
            .iter()
            .min_by_key(|v| if v[3] == 1 { v[1] } else { v[0] })
            .unwrap()
            .clone();
        let min_x = if min_x[3] == 1 { min_x[1] } else { min_x[0] };

        let max_x = new_input
            .iter()
            .max_by_key(|v| if v[3] == 1 { v[2] } else { v[0] })
            .unwrap()
            .clone();
        let max_x = if max_x[3] == 1 { max_x[2] } else { max_x[0] };

        Day17 {
            input: new_input,
            min_y,
            max_y,
            min_x,
            max_x,
        }
    }

    fn build_map(&self) -> Vec<Vec<char>> {
        let mut map: Vec<Vec<char>> = Vec::with_capacity(self.max_y - self.min_y + 1);
        for _ in 0..map.capacity() {
            let mut row: Vec<char> = Vec::with_capacity(self.max_x - self.min_x + 5);
            for _ in 0..row.capacity() {
                row.push('.');
            }
            map.push(row);
        }
        for line in &self.input {
            for i in line[1]..line[2] + 1 {
                if line[3] == 0 {
                    map[i - self.min_y][line[0] - self.min_x + 2] = '#';
                } else {
                    map[line[0] - self.min_y][i - self.min_x + 2] = '#';
                }
            }
        }
        map
    }

    fn tick(map: &mut Vec<Vec<char>>, min_x: usize, visited: &mut Vec<Vec<bool>>) {
        let (x, y) = (500 - min_x + 2, 0);
        Day17::flow_down(x, y, map, visited);
    }

    fn split(
        dir: isize,
        mut x: usize,
        y: usize,
        map: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        while x < map[y].len() - 1
            && x > 0
            && map[y][x] != '#'
            && map[y + 1][x] != '.'
            && map[y + 1][x] != '|'
        {
            if map[y][x] == '.' {
                map[y][x] = '|';
            }
            x = (x as isize + dir) as usize;
        }
        if map[y][x] == '#' {
            map[y][(x as isize - dir) as usize] = '~';
            return true;
        } else {
            Day17::flow_down(x, y, map, visited);
            return false;
        }
    }

    fn flow_down(
        mut x: usize,
        mut y: usize,
        map: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) {
        if visited[y][x] {
            return;
        }
        while y < map.len() - 1 && map[y + 1][x] != '~' && map[y + 1][x] != '#' {
            if map[y][x] == '.' {
                map[y][x] = '|';
            }
            visited[y][x] = true;
            y += 1;
        }
        if y > map.len() - 2 && (map[y][x] == '.' || map[y][x] == '|') {
            map[y][x] = '|';
            return;
        }
        let left = Day17::split(-1, x, y, map, visited);
        let right = Day17::split(1, x, y, map, visited);
        if left && right {
            x += 1;
            if map[y][x] != '#' {
                while x < map[y].len() - 1 && map[y][x] != '~' {
                    map[y][x] = '~';
                    x += 1;
                }
            }
            x -= 1;
            while map[y][x] != '|' && map[y][x] != '#' {
                x -= 1;
            }
            if map[y][x] != '#' {
                while map[y][x] != '~' {
                    map[y][x] = '~';
                    x -= 1;
                }
            }
        }
        if !left && right {
            while x < map[y].len() - 1 && map[y][x] != '~' {
                x += 1;
            }
            map[y][x] = '|';
        }
        if !right && left {
            while x > 0 && map[y][x] != '~' {
                x -= 1;
            }
            map[y][x] = '|';
        }
    }

    fn execute(&self) -> Vec<Vec<char>> {
        let mut map = self.build_map();
        map[0][500 - self.min_x + 2] = '|';
        let min_x = self.min_x;
        let mut prev_map = map.clone();
        let mut visited: Vec<Vec<bool>> = Vec::with_capacity(map.len());
        for row in &map {
            let mut vec: Vec<bool> = Vec::with_capacity(row.len());
            for _ in row {
                vec.push(false);
            }
            visited.push(vec);
        }
        loop {
            for row in &mut visited {
                for value in row.iter_mut() {
                    *value = false;
                }
            }
            Day17::tick(&mut map, min_x, &mut visited);
            if map.iter().eq(prev_map.iter()) {
                break;
            }
            prev_map = map.clone();
        }
        map
    }
}

impl Day for Day17 {
    fn get_part_a_result(&self) -> String {
        let map = self.execute();
        let mut result = 0;
        for row in &map {
            for c in row {
                if *c == '~' || *c == '|' {
                    result += 1;
                }
            }
        }
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let map = self.execute();
        let mut result = 0;
        for row in &map {
            for c in row {
                if *c == '~' {
                    result += 1;
                }
            }
        }
        result.to_string()
    }
}
