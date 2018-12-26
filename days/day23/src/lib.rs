extern crate utils;

use utils::Day;
use std::collections::BTreeMap;

pub struct Day23 {
    input: Vec<Vec<isize>>,
}

impl Day23 {
    pub fn new(input: String) -> Day23 {
        let input = utils::extract_integers_from_string(&input);
        Day23 { input }
    }

    fn get_strongest_nanobot(&self) -> usize {
        self.input.iter().enumerate().map(|(i, bot)| (i, bot[3])).max_by_key(|bot| bot.1).unwrap().clone().0 as usize
    }

    fn calculate_distances(&self, vec: &mut Vec<Vec<isize>>, strongest_bot: usize) {
        let strongest_bot = vec[strongest_bot].clone();
        for nanobot in vec {
            let distance = (nanobot[0] - strongest_bot[0]).abs() + (nanobot[1] - strongest_bot[1]).abs() + (nanobot[2] - strongest_bot[2]).abs();
            if nanobot.len() < 5 {
                nanobot.push(distance);
            } else {
                nanobot[4] = distance;
            }
        }
    }
}

impl Day for Day23 {
    fn get_part_a_result(&self) -> String {
        let strongest_bot = self.get_strongest_nanobot();
        let mut vec = self.input.clone();
        self.calculate_distances(&mut vec, strongest_bot);
        let strongest_distance = vec[strongest_bot][3];
        let mut result = 0;
        for nanobot in vec {
            if nanobot[4] <= strongest_distance {
                result += 1;
            }
        }
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let mut map: BTreeMap<isize, isize> = BTreeMap::new();
        for bot in self.input.iter() {
            let min = bot[0] + bot[1] + bot[2] - bot[3];
            *map.entry(min).or_insert(0) += 1;
            let max = bot[0] + bot[1] + bot[2] + bot[3] + 1;
            *map.entry(max).or_insert(0) -= 1;
        }
        let mut max = 0;
        let mut max_beg = 0;
        map.iter().fold(0, |acc, x| {
            let result = acc + *x.1;
            if result > max {
                max = result;
                max_beg = *x.0;
            }
            result
        });
        let max_end = map.into_iter().skip_while(|x| x.0 <= max_beg).next().unwrap().0 - 1;
        max_end.to_string()
    }
}
