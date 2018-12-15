extern crate utils;

use std::cmp::Ordering;
use utils::Day;

pub struct Day15 {
    map: Vec<Vec<char>>,
    units: Vec<Unit>,
}

#[derive(Copy, Debug, Eq)]
struct Unit {
    x: usize,
    y: usize,
    hp: i16,
    attack_power: i16,
    is_elf: bool,
}

impl Clone for Unit {
    fn clone(&self) -> Unit {
        *self
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Unit) -> bool {
        self.x == other.x && self.y == other.y && self.hp == other.hp && self.is_elf == other.is_elf
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Unit) -> Ordering {
        let order = self.y.cmp(&other.y);
        if order == Ordering::Equal {
            return self.x.cmp(&other.x);
        }
        order
    }
}

impl Day15 {
    pub fn new(input: String) -> Day15 {
        let mut map = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut units: Vec<Unit> = Vec::new();
        for (i, y) in map.iter_mut().enumerate() {
            for (j, x) in y.iter_mut().enumerate() {
                let is_elf = match *x {
                    'G' => Some(false),
                    'E' => Some(true),
                    _ => None,
                };
                if let Some(is_elf) = is_elf {
                    units.push(Unit {
                        x: j,
                        y: i,
                        hp: 200,
                        attack_power: 3,
                        is_elf,
                    });
                }
            }
        }
        Day15 { map, units }
    }

    fn fill_adjacent_node_distance(
        &self,
        (x, y): (usize, usize),
        distance: u16,
        distance_map: &mut Vec<Vec<u16>>,
        map: &Vec<Vec<char>>,
    ) {
        if distance < distance_map[y][x] {
            distance_map[y][x] = distance;
        }
        let next_distance = distance + 1;
        if y != 0 && map[y - 1][x] == '.' && next_distance < distance_map[y - 1][x] {
            self.fill_adjacent_node_distance((x, y - 1), next_distance, distance_map, map);
        }
        if x + 1 < map[y].len() && map[y][x + 1] == '.' && next_distance < distance_map[y][x + 1] {
            self.fill_adjacent_node_distance((x + 1, y), next_distance, distance_map, map);
        }
        if y + 1 < map.len() && map[y + 1][x] == '.' && next_distance < distance_map[y + 1][x] {
            self.fill_adjacent_node_distance((x, y + 1), next_distance, distance_map, map);
        }
        if x != 0 && map[y][x - 1] == '.' && next_distance < distance_map[y][x - 1] {
            self.fill_adjacent_node_distance((x - 1, y), next_distance, distance_map, map);
        }
    }

    fn get_next_target(
        &self,
        unit: &Unit,
        units: &Vec<Unit>,
        map: &Vec<Vec<char>>,
    ) -> (usize, usize) {
        let mut distance_map: Vec<Vec<u16>> = Vec::with_capacity(map.len());
        for y in map {
            let mut row = Vec::with_capacity(y.len());
            for _ in y {
                row.push(std::u16::MAX);
            }
            distance_map.push(row);
        }
        self.fill_adjacent_node_distance((unit.x, unit.y), 0, &mut distance_map, map);
        let is_target_elf = !unit.is_elf;
        let (mut x, mut y) = (0, 0);
        let mut min_distance = std::u16::MAX;
        for other in units {
            if other.is_elf == is_target_elf {
                if other.y != 0 && distance_map[other.y - 1][other.x] < min_distance {
                    min_distance = distance_map[other.y - 1][other.x];
                    x = other.x;
                    y = other.y - 1;
                }
                if other.x != 0 && distance_map[other.y][other.x - 1] < min_distance {
                    min_distance = distance_map[other.y][other.x - 1];
                    x = other.x - 1;
                    y = other.y;
                }
                if other.x + 1 < map[0].len() && distance_map[other.y][other.x + 1] < min_distance {
                    min_distance = distance_map[other.y][other.x + 1];
                    x = other.x + 1;
                    y = other.y;
                }
                if other.y + 1 < map.len() && distance_map[other.y + 1][other.x] < min_distance {
                    min_distance = distance_map[other.y + 1][other.x];
                    x = other.x;
                    y = other.y + 1;
                }
            }
        }
        for (i, y) in map.iter().enumerate() {
            for (j, _) in y.iter().enumerate() {
                distance_map[i][j] = std::u16::MAX;
            }
        }
        self.fill_adjacent_node_distance((x, y), 0, &mut distance_map, map);
        min_distance = std::u16::MAX;
        let (mut move_to_x, mut move_to_y) = (0, 0);
        if unit.y != 0 && distance_map[unit.y - 1][unit.x] < min_distance {
            min_distance = distance_map[unit.y - 1][unit.x];
            move_to_x = unit.x;
            move_to_y = unit.y - 1;
        }
        if unit.x != 0 && distance_map[unit.y][unit.x - 1] < min_distance {
            min_distance = distance_map[unit.y][unit.x - 1];
            move_to_x = unit.x - 1;
            move_to_y = unit.y;
        }
        if unit.x + 1 < map[0].len() && distance_map[unit.y][unit.x + 1] < min_distance {
            min_distance = distance_map[unit.y][unit.x + 1];
            move_to_x = unit.x + 1;
            move_to_y = unit.y;
        }
        if unit.y + 1 < map.len() && distance_map[unit.y + 1][unit.x] < min_distance {
            min_distance = distance_map[unit.y + 1][unit.x];
            move_to_x = unit.x;
            move_to_y = unit.y + 1;
        }
        if min_distance == std::u16::MAX {
            return (unit.x, unit.y);
        }
        (move_to_x, move_to_y)
    }

    fn in_battle(&self, unit: &Unit, units: &Vec<Unit>, map: &Vec<Vec<char>>) -> Option<usize> {
        let mut enemy = None;
        let enemy_symbol = if unit.is_elf { 'G' } else { 'E' };
        let mut min_hp = std::i16::MAX;
        if unit.y != 0 && map[unit.y - 1][unit.x] == enemy_symbol {
            for (k, other) in units.iter().enumerate() {
                if other.x == unit.x && other.y == unit.y - 1 && other.hp < min_hp {
                    min_hp = other.hp;
                    enemy = Some(k);
                    break;
                }
            }
        }
        if unit.x != 0 && map[unit.y][unit.x - 1] == enemy_symbol {
            for (k, other) in units.iter().enumerate() {
                if other.x == unit.x - 1 && other.y == unit.y && other.hp < min_hp {
                    min_hp = other.hp;
                    enemy = Some(k);
                    break;
                }
            }
        }
        if unit.x + 1 < map[unit.y].len() && map[unit.y][unit.x + 1] == enemy_symbol {
            for (k, other) in units.iter().enumerate() {
                if other.x == unit.x + 1 && other.y == unit.y && other.hp < min_hp {
                    min_hp = other.hp;
                    enemy = Some(k);
                    break;
                }
            }
        }
        if unit.y + 1 < map.len() && map[unit.y + 1][unit.x] == enemy_symbol {
            for (k, other) in units.iter().enumerate() {
                if other.x == unit.x && other.y == unit.y + 1 && other.hp < min_hp {
                    enemy = Some(k);
                    break;
                }
            }
        }
        enemy
    }
}

impl Day for Day15 {
    fn get_part_a_result(&self) -> String {
        let mut units = self.units.clone();
        let mut map = self.map.clone();
        let mut rounds_done = 0;
        let mut i = 0;
        units.sort();
        while units.iter().filter(|u| u.is_elf == true).count() != 0
            && units.iter().filter(|u| u.is_elf == false).count() != 0
        {
            if let Some(enemy) = self.in_battle(&units[i], &units, &map) {
                units[enemy].hp -= units[i].attack_power;
                if units[enemy].hp <= 0 {
                    map[units[enemy].y][units[enemy].x] = '.';
                    units.remove(enemy);
                    if i > enemy {
                        i -= 1;
                    }
                }
            } else {
                map[units[i].y][units[i].x] = '.';
                let (t_x, t_y) = self.get_next_target(&units[i], &units, &map);
                units[i].x = t_x;
                units[i].y = t_y;
                map[t_y][t_x] = if units[i].is_elf { 'E' } else { 'G' };
                if let Some(enemy) = self.in_battle(&units[i], &units, &map) {
                    units[enemy].hp -= units[i].attack_power;
                    if units[enemy].hp <= 0 {
                        map[units[enemy].y][units[enemy].x] = '.';
                        units.remove(enemy);
                    }
                }
            }

            i += 1;
            if i >= units.len() {
                units.sort();
                rounds_done += 1;
                i = 0;
            }
        }
        let hp_sum: usize = units.into_iter().map(|unit| unit.hp as usize).sum();
        let result: usize = hp_sum * rounds_done;
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let mut attack_power = 3;
        'outer: loop {
            attack_power += 1;
            let mut units = self.units.clone();
            let mut map = self.map.clone();
            for unit in &mut units {
                if unit.is_elf {
                    unit.attack_power = attack_power;
                }
            }
            let mut rounds_done = 0;
            let mut i = 0;
            units.sort();
            while units.iter().filter(|u| u.is_elf == false).count() != 0 {
                if let Some(enemy) = self.in_battle(&units[i], &units, &map) {
                    units[enemy].hp -= units[i].attack_power;
                    if units[enemy].hp <= 0 {
                        if units[enemy].is_elf {
                            continue 'outer;
                        }
                        map[units[enemy].y][units[enemy].x] = '.';
                        units.remove(enemy);
                        if i > enemy {
                            i -= 1;
                        }
                    }
                } else {
                    map[units[i].y][units[i].x] = '.';
                    let (t_x, t_y) = self.get_next_target(&units[i], &units, &map);
                    units[i].x = t_x;
                    units[i].y = t_y;
                    map[t_y][t_x] = if units[i].is_elf { 'E' } else { 'G' };
                    if let Some(enemy) = self.in_battle(&units[i], &units, &map) {
                        units[enemy].hp -= units[i].attack_power;
                        if units[enemy].hp <= 0 {
                            map[units[enemy].y][units[enemy].x] = '.';
                            units.remove(enemy);
                            if i > enemy {
                                i -= 1;
                            }
                        }
                    }
                }

                i += 1;
                if i >= units.len() {
                    units.sort();
                    rounds_done += 1;
                    i = 0;
                }
            }
            let hp_sum: usize = units.into_iter().map(|unit| unit.hp as usize).sum();
            let result: usize = hp_sum * rounds_done;
            return result.to_string();
        }
    }
}
