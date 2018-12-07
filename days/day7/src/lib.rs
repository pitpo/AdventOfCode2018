extern crate utils;

use std::collections::BTreeSet;
use std::collections::HashMap;
use utils::Day;

pub struct Day7 {
    input: String,
    num_of_workers: usize,
    task_base_length: u8,
}

impl Day7 {
    pub fn new(input: String) -> Day7 {
        Day7 {
            input,
            num_of_workers: 5,
            task_base_length: 60,
        }
    }

    pub fn with_options(input: String, num_of_workers: usize, task_base_length: u8) -> Day7 {
        Day7 {
            input,
            num_of_workers,
            task_base_length,
        }
    }

    fn parse_input(&self) -> HashMap<char, Vec<char>> {
        let mut task_list: BTreeSet<char> = BTreeSet::new();
        let mut result: HashMap<char, Vec<char>> = HashMap::new();
        let vec: Vec<Vec<char>> = self
            .input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter(|s| s.len() == 1)
                    .map(|s| {
                        let c = s.chars().next().unwrap();
                        task_list.insert(c);
                        c
                    }).collect()
            }).collect();
        task_list.into_iter().for_each(|task| {
            let mut requirements: Vec<char> = Vec::new();
            for req in &vec {
                if req[1] == task {
                    requirements.push(req[0]);
                }
            }
            result.insert(task, requirements);
        });
        result
    }

    fn task_can_be_started(&self, requirements: &Vec<char>, tasks_done: &String) -> bool {
        let mut can_be_started = true;
        for req in requirements {
            if !tasks_done.contains(*req) {
                can_be_started = false;
                break;
            }
        }
        can_be_started
    }
}

impl Day for Day7 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let mut done = String::new();
        let mut can_be_done: BTreeSet<char> = BTreeSet::new();
        loop {
            for (task, requirements) in &input {
                if !done.contains(*task) && self.task_can_be_started(requirements, &done) {
                    can_be_done.insert(*task);
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
        let mut timer = 0;
        let mut workers: HashMap<char, u8> = HashMap::new();
        let mut workers_to_remove: Vec<char> = Vec::new();
        loop {
            for (task, requirements) in &input {
                if !done.contains(*task)
                    && !workers.contains_key(task)
                    && self.task_can_be_started(requirements, &done)
                {
                    can_be_done.insert(*task);
                }
            }
            while !can_be_done.is_empty() && workers.len() < self.num_of_workers {
                let task = can_be_done.iter().next().unwrap().clone();
                can_be_done.remove(&task);
                workers.insert(task, self.task_base_length + (task as u8 - 64));
            }
            if can_be_done.is_empty() && workers.is_empty() {
                break;
            }
            for (task, time_left) in &mut workers {
                if *time_left > 1 {
                    *time_left -= 1;
                } else {
                    done.push(*task);
                    workers_to_remove.push(*task);
                }
            }
            if !workers_to_remove.is_empty() {
                for task in &workers_to_remove {
                    workers.remove(task);
                }
                workers_to_remove = Vec::new()
            }
            timer += 1;
        }
        timer.to_string()
    }
}
