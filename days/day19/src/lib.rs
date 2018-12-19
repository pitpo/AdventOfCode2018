extern crate utils;

use utils::Day;

struct Instruction {
    name: String,
    resulting_register: usize,
    a_value: usize,
    b_value: usize,
}

pub struct Day19 {
    instruction_register: usize,
    instructions: Vec<Instruction>,
}

impl Day19 {
    pub fn new(input: String) -> Day19 {
        let mut instruction_register = 0;
        let instructions = input.lines().enumerate().filter_map(|line| {
            let vec = line.1.split(' ').collect::<Vec<&str>>();
            if line.0 == 0 {
                instruction_register = vec[1].parse().unwrap();
                return None;
            } else {
                let name = vec[0].to_string();
                let a_value = vec[1].parse().unwrap();
                let b_value = vec[2].parse().unwrap();
                let resulting_register = vec[3].parse().unwrap();
                return Some(Instruction { name, resulting_register, a_value, b_value });
            }
        }).collect::<Vec<Instruction>>();
        Day19 { instruction_register, instructions }
    }

    fn add(&self, val_a: usize, val_b: usize) -> usize {
        val_a + val_b
    }

    fn mul(&self, val_a: usize, val_b: usize) -> usize {
        val_a * val_b
    }

    fn ban(&self, val_a: usize, val_b: usize) -> usize {
        val_a & val_b
    }

    fn bor(&self, val_a: usize, val_b: usize) -> usize {
        val_a | val_b
    }

    fn set(&self, val_a: usize, _val_b: usize) -> usize {
        val_a
    }

    fn gtr(&self, val_a: usize, val_b: usize) -> usize {
        if val_a > val_b {1} else {0}
    }

    fn eq(&self, val_a: usize, val_b: usize) -> usize {
        if val_a == val_b {1} else {0}
    }

    fn do_operation(&self, ins: &Instruction, register: &Vec<usize>) -> usize {
        match ins.name.as_str() {
            "addr" => self.add(register[ins.a_value], register[ins.b_value]),
            "addi" => self.add(register[ins.a_value], ins.b_value),
            "mulr" => self.mul(register[ins.a_value], register[ins.b_value]),
            "muli" => self.mul(register[ins.a_value], ins.b_value),
            "banr" => self.ban(register[ins.a_value], register[ins.b_value]),
            "bani" => self.ban(register[ins.a_value], ins.b_value),
            "borr" => self.bor(register[ins.a_value], register[ins.b_value]),
            "bori" => self.bor(register[ins.a_value], ins.b_value),
            "setr" => self.set(register[ins.a_value], ins.b_value),
            "seti" => self.set(ins.a_value, ins.b_value),
            "gtrr" => self.gtr(register[ins.a_value], register[ins.b_value]),
            "gtri" => self.gtr(register[ins.a_value], ins.b_value),
            "gtir" => self.gtr(ins.a_value, register[ins.b_value]),
            "eqrr" => self.eq(register[ins.a_value], register[ins.b_value]),
            "eqri" => self.eq(register[ins.a_value], ins.b_value),
            "eqir" => self.eq(ins.a_value, register[ins.b_value]),
            _ => {0}
        }
    }
}

impl Day for Day19 {
    fn get_part_a_result(&self) -> String {
        let mut register = vec![0, 0, 0, 0, 0, 0];
        loop {
            let ins = &self.instructions[register[self.instruction_register]];
            register[ins.resulting_register] = self.do_operation(&ins, &register);
            register[self.instruction_register] += 1;
            if register[self.instruction_register] >= self.instructions.len() {
                break;
            }
        }
        register[0].to_string()
    }
    fn get_part_b_result(&self) -> String {
        let mut register = vec![1, 0, 0, 0, 0, 0];
        loop {
            let ins = &self.instructions[register[self.instruction_register]];
            register[ins.resulting_register] = self.do_operation(&ins, &register);
            register[self.instruction_register] += 1;
            if register[self.instruction_register] == 1 {
                break;
            }
        }
        let mut result = 0;
        for i in 1..f64::sqrt(register[4] as f64) as usize {
            if register[4] % i == 0 {
                result += i;
                if register[4] / i != i {
                    result += register[4]/i;
                }
            }
        }
        result.to_string()
    }
}
