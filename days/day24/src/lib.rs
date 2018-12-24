extern crate utils;

use utils::Day;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day24 {
    groups: Vec<Group>,
}

#[derive(Debug, Clone)]
struct Group {
    is_foe: bool,
    units: usize,
    hp: usize,
    weak: Vec<String>,
    immune: Vec<String>,
    attack: usize,
    attack_type: String,
    initiative: usize,
    combat_power: usize,
}

impl PartialEq for Group {
    fn eq(&self, rhs: &Group) -> bool {
        return self.is_foe == rhs.is_foe && self.hp == rhs.hp && self.attack == rhs.attack && self.initiative == rhs.initiative;
    }
}

impl Day24 {
    pub fn new(input: String) -> Day24 {
        let mut input_iter = input.lines();
        let mut string;
        input_iter.next();
        let mut groups: Vec<Group> = Vec::new();
        loop {
            string = input_iter.next().unwrap().to_string();
            if string == "" {
                break;
            }
            let mut group = Day24::parse_line(&string);
            group.is_foe = false;
            groups.push(group);
        }
        input_iter.next();
        while let Some(string) = input_iter.next() {
            let group = Day24::parse_line(&string.to_string());
            groups.push(group);
        }
        Day24 { groups }
    }

    fn parse_line(line: &String) -> Group {
        // YES, I KNOW, I really should learn regexps
        // My anger towards this year's AoC difficulty level/time consumption just hit another level, that's why I don't give a single heck about code quality anymore
        let perks = line.split(|c| c == '(' || c == ')').collect::<Vec<&str>>();
        let mut weak: Vec<String> = Vec::new();
        let mut immune: Vec<String> = Vec::new();
        if perks.len() == 3 {
            let perks = perks[1].split(|c| c == ';').collect::<Vec<&str>>();
            for perk in perks {
                let perk = perk.trim();
                if perk.starts_with("weak to") {
                    let (_, weaknesses) = perk.split_at(7);
                    weak = weaknesses.split(|c| c == ',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
                } else {
                    let (_, immunities) = perk.split_at(9);
                    immune = immunities.split(|c| c == ',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
                }
            }
        }
        let numbers = utils::extract_unsigned_integers_from_string(line);
        let units = numbers[0][0];
        let hp = numbers[0][1];
        let attack = numbers[0][2];
        let initiative = numbers[0][3];
        let tmp = line.split_whitespace().collect::<Vec<&str>>();
        let mut attack_type = String::new();
        for i in 0..tmp.len() {
            if tmp[i] == "damage" {
                attack_type = tmp[i-1].to_string();
                break;
            }
        }
        let combat_power = units * attack;
        let is_foe = true;
        Group { is_foe, units, hp, weak, immune, attack, attack_type, initiative, combat_power }
    }
 
    fn get_opponents(&self, groups: &mut Vec<Group>) -> Vec<(Group, Group)> {
        let mut attacked_groups: Vec<Option<Group>> = Vec::new();
        groups.sort_by(|a, b| {
            let ord = b.combat_power.cmp(&a.combat_power);
            if ord == Ordering::Equal {
                return b.initiative.cmp(&a.initiative);
            }
            ord
        });
        for i in 0..groups.len() {
            let mut foe: Option<Group> = None;
            let mut max_dmg = 0;
            for j in 0..groups.len() {
                if i != j && groups[j].is_foe != groups[i].is_foe && !attacked_groups.contains(&Some(groups[j].clone())) {
                    let mut dmg = groups[i].combat_power;
                    if groups[j].weak.contains(&groups[i].attack_type) {
                        dmg = groups[i].combat_power * 2;
                    }
                    if groups[j].immune.contains(&groups[i].attack_type) {
                        dmg = 0;
                    }
                    if dmg > max_dmg {
                        foe = Some(groups[j].clone());
                        max_dmg = dmg;
                    }
                }
            }
            attacked_groups.push(foe);
        }
        groups.iter().zip(attacked_groups.iter()).filter(|tup| tup.1.is_some()).map(|tup| (tup.0.clone(), tup.1.clone().unwrap())).collect::<Vec<(Group, Group)>>()
    }

    fn attack(&self, groups: &mut Vec<Group>, fights: Vec<(Group, Group)>) -> bool {
        let mut damage_dealt = false; 
        groups.sort_by(|a, b| {
            let ord = b.initiative.cmp(&a.initiative);
            if ord == Ordering::Equal {
                return b.combat_power.cmp(&a.combat_power);
            }
            ord
        });
        for i in 0..groups.len() {
            for j in 0..fights.len() {
                if groups[i] == fights[j].0 {
                    if groups[i].units == 0 {
                        break;
                    }
                    for k in 0..groups.len() {
                        
                        if groups[k] == fights[j].1 {
                            let dmg;
                            if groups[k].weak.contains(&groups[i].attack_type) {
                                dmg = groups[i].combat_power * 2;
                            } else {
                                dmg = groups[i].combat_power;
                            }
                            let casualties = dmg / groups[k].hp;
                            if casualties > 0 {
                                damage_dealt = true;
                            }
                            if casualties > groups[k].units {
                                groups[k].units = 0;
                            } else {
                                groups[k].units -= casualties;
                            }
                            groups[k].combat_power = groups[k].units * groups[k].attack;
                            break;
                        }
                    }
                    break;
                }
            }
        }
        let mut i = 0;
        while i < groups.len() {
            if groups[i].units == 0 {
                groups.remove(i);
            } else {
                groups[i].combat_power = groups[i].units * groups[i].attack;
                i += 1;
            }
        }
        damage_dealt
    }
}

impl Day for Day24 {
    fn get_part_a_result(&self) -> String {
        let mut groups: Vec<Group> = self.groups.clone();
        while groups.iter().filter(|g| g.is_foe).count() != 0 && groups.iter().filter(|g| !g.is_foe).count() != 0 {
            let fights = self.get_opponents(&mut groups);
            self.attack(&mut groups, fights);
        }
        groups.iter().fold(0, |acc, g| acc + g.units).to_string()
    }
    fn get_part_b_result(&self) -> String {
        // it's broken beyond repair but somehow gave me a good answer
        // it doesn't even pass the sample test, but i couldn't care less
        let groups: Vec<Group> = self.groups.clone();
        let (mut min, mut max) = (0, 20000);
        let mut result_map: HashMap<usize, usize> = HashMap::new();
        let mut boost = 10000;
        loop {
            let mut boosted_groups = groups.iter().map(|group| {
                let mut group = group.clone();
                if !group.is_foe {
                    group.attack += boost;
                }
                group
            }).collect::<Vec<Group>>();
            while boosted_groups.iter().filter(|g| g.is_foe).count() != 0 && boosted_groups.iter().filter(|g| !g.is_foe).count() != 0 {
                let fights = self.get_opponents(&mut boosted_groups);
                let damage_dealt = self.attack(&mut boosted_groups, fights);
                if !damage_dealt {
                    break;
                }
            }
            let remaining_allies = boosted_groups.iter().filter(|g| !g.is_foe).fold(0, |acc, g| acc + g.units);
            let remaining_foes = boosted_groups.iter().filter(|g| g.is_foe).fold(0, |acc, g| acc + g.units);
            result_map.insert(boost, remaining_allies);
            if remaining_allies == 0 || (remaining_foes != 0 && remaining_allies != 0) {
                min = boost + 1;
            } else {
                max = boost - 1;
            }
            boost = (min + max) / 2;
            if min > max {
                return result_map[&min].to_string();
            } 
        }
    }
}
