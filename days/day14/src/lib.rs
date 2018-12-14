extern crate utils;

use utils::Day;

pub struct Day14 {
    input: usize
}

impl Day14 {
    pub fn new(input: String) -> Day14 {
        let input = input.trim().parse().unwrap();
        Day14 { input }
    }

    fn split_input(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut input = self.input;
        while input > 0 {
            result.push((input%10) as u8);
            input /= 10;
        }
        result
    }

    fn tick(&self, arr: &mut Vec<u8>, first_elf: &mut usize, second_elf: &mut usize) {
        let score = arr[*first_elf] + arr[*second_elf];
        if score >= 10 {
            arr.push(score/10);
        }
        arr.push(score%10);
        *first_elf = (*first_elf + 1 + (arr[*first_elf] as usize)) % arr.len();
        *second_elf = (*second_elf + 1 + (arr[*second_elf] as usize)) % arr.len();
    }

    fn get_sequence_start(&self, arr: &Vec<u8>, sequence: &Vec<u8>) -> Option<usize> {
        for i in (arr.len()-2..arr.len()).rev() {
            if arr[i] == sequence[0] {
                let mut j = 1;
                while j < sequence.len() && i - j < arr.len() && arr[i-j] == sequence[j] {
                    j += 1;
                }
                if j >= sequence.len() {
                    return Some(i-j+1);
                }
            }
        }
        return None;
    }
}

impl Day for Day14 {
    fn get_part_a_result(&self) -> String {
        let mut arr: Vec<u8> = Vec::with_capacity(self.input + 20);
        arr.push(3);
        arr.push(7);
        let (mut first_elf, mut second_elf) = (0, 1);
        while arr.len() < self.input + 11 {
            self.tick(&mut arr, &mut first_elf, &mut second_elf);
        }
        let mut result = String::with_capacity(10);
        for i in self.input..self.input+10 {
            result.push(std::char::from_digit(arr[i] as u32, 10).unwrap());
        }
        result
    }
    fn get_part_b_result(&self) -> String {
        let mut arr: Vec<u8> = Vec::with_capacity(self.input * 5);
        arr.push(3);
        arr.push(7);
        let (mut first_elf, mut second_elf) = (0, 1);
        let input = self.split_input();
        loop {
            self.tick(&mut arr, &mut first_elf, &mut second_elf);
            if let Some(i) = self.get_sequence_start(&arr, &input) {
                return i.to_string();
            }
        }
    }
}
