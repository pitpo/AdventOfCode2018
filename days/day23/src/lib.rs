extern crate utils;

use utils::Day;

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
        // I give up
        String::new()
    }
}
