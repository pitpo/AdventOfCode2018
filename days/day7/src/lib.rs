extern crate utils;

use utils::Day;
use std::collections::BTreeSet;

pub struct Day7 {
    input: String,
}

impl Day7 {
    pub fn new(input: String) -> Day7 {
        Day7 { input }
    }

    fn parse_input(&self) -> Vec<Vec<char>> {
        self.input.lines().map(|line| line.split_whitespace().filter(|s| s.len() == 1).map(|s| s.chars().next().unwrap()).collect()).collect()
    }

}

// it might be one of the worst things I've ever done, gotta definitely rework it soon
impl Day for Day7 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let mut done = String::new();
        let mut can_be_done: BTreeSet<char> = BTreeSet::new();
        for i in 0..input.len() {
            let mut has_requirements = false;
            for j in 0..input.len() {
                if input[i][0] == input[j][1] {
                    has_requirements = true;
                }
            }
            if !has_requirements {
                can_be_done.insert(input[i][0]);
            }
        }
        loop {
            for i in 0..input.len() {
                let mut can_start = true;
                for j in 0..input.len() {
                    if input[i][1] == input[j][1] && (done.contains(input[j][1]) || !done.contains(input[j][0])) {
                        can_start = false;
                        break;
                    }
                }
                if can_start {
                    can_be_done.insert(input[i][1]);
                }
            }
            if !can_be_done.is_empty() {
                let task = can_be_done.iter().next().unwrap().clone();
                can_be_done.remove(&task);
                done.push(task);
            } else {
                break;
            }
        }
        done.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let mut done = String::new();
        let mut can_be_done: BTreeSet<char> = BTreeSet::new();
        let mut workers = vec![(' ', 0), (' ', 0), (' ', 0), (' ', 0), (' ',0)];
        let mut timer = 0;
        for i in 0..input.len() {
            let mut has_requirements = false;
            for j in 0..input.len() {
                if input[i][0] == input[j][1] {
                    has_requirements = true;
                }
            }
            if !has_requirements {
                can_be_done.insert(input[i][0]);
            }
        }
        // at this point, after a sleepless night (thanks The Game Awards), I don't even want to know what the actual heck is happening down there, it works and takes "only" 10ms to get the solution, but it's horrible
        loop {
            for i in 0..input.len() {
                let mut can_start = true;
                for j in 0..workers.len() {
                    if workers[j].0 == input[i][1] {
                        can_start = false;
                    }
                }
                for j in 0..input.len() {
                    if input[i][1] == input[j][1] && (done.contains(input[j][1]) || !done.contains(input[j][0])) {
                        can_start = false;
                        break;
                    }
                }
                if can_start {
                    can_be_done.insert(input[i][1]);
                }
            }
            if !can_be_done.is_empty() {
                for i in 0..workers.len() {
                    if workers[i].1 == 0 && !can_be_done.is_empty() {
                        let task = can_be_done.iter().next().unwrap().clone();
                        can_be_done.remove(&task);
                        workers[i].0 = task;
                        workers[i].1 = 60 + task as u8 - 64;
                    }
                }
            } else {
                let mut job_done = true;
                for i in 0..workers.len() {
                    if workers[i].1 > 0 {
                        job_done = false;
                        break;
                    }
                }
                if job_done {
                    break;
                }
            }
            for i in 0..workers.len() {
                if workers[i].1 > 1 {
                    workers[i].1 -= 1;
                } else if workers[i].1 == 1 {
                    workers[i].1 = 0;
                    done.push(workers[i].0);
                }
            }
            timer += 1;
        }
        timer.to_string()
    }
}
