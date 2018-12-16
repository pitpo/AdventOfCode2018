extern crate utils;

use utils::Day;

pub struct Day16 {
    examples: Vec<Vec<Vec<usize>>>,
    program: Vec<Vec<usize>>,
    op_map: Vec<String>,
}

impl Day16 {
    pub fn new(input: String) -> Day16 {
        let mut examples = Vec::new();
        let mut tmp = Vec::new();
        let mut split_point = 0;
        for (i, line) in input.lines().enumerate() {
            if i % 4 == 3 {
                examples.push(tmp.clone());
                continue;
            }
            if i % 4 == 0 {
                if line.is_empty() {
                    split_point = i;
                    break;
                }
                tmp = Vec::new();
            }
            tmp.push(utils::extract_unsigned_integers_from_string(&line.to_string()).get(0).unwrap().clone());
        }
        let program = input.lines().skip(split_point).filter(|line| !line.is_empty()).map(|line| line.split(" ").filter_map(|num| num.trim().parse().ok()).collect::<Vec<usize>>()).collect();
        let op_map = vec!["bori", "borr", "addi", "muli", "addr", "bani", "gtri", "setr", "gtrr", "seti", "eqir", "eqrr", "mulr", "eqri", "gtir", "banr"];
        let op_map = op_map.into_iter().map(|s| s.to_string()).collect();
        Day16 { examples, program, op_map }
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
}

impl Day for Day16 {
    fn get_part_a_result(&self) -> String {
        let mut result = 0;
        for example in &self.examples {
            let mut acc = 0;
            let before = &example[0];
            let input = &example[1];
            let after = &example[2];
            let mut vec: Vec<&str> = Vec::new();
            if input[2] < 4 && self.add(before[input[1]], before[input[2]]) == after[input[3]] {acc += 1; vec.push("addr");}
            if self.add(before[input[1]], input[2]) == after[input[3]] {acc += 1; vec.push("addi");}
            if input[2] < 4 && self.mul(before[input[1]], before[input[2]]) == after[input[3]] {acc += 1; vec.push("mulr");}
            if self.mul(before[input[1]], input[2]) == after[input[3]] {acc += 1; vec.push("muli");}
            if input[2] < 4 && self.ban(before[input[1]], before[input[2]]) == after[input[3]] {acc += 1; vec.push("banr");}
            if self.ban(before[input[1]], input[2]) == after[input[3]] {acc += 1; vec.push("bani");}
            if input[2] < 4 && self.bor(before[input[1]], before[input[2]]) == after[input[3]] {acc += 1; vec.push("borr");}
            if self.bor(before[input[1]], input[2]) == after[input[3]] {acc += 1; vec.push("bori");}
            if input[1] < 4 && self.set(before[input[1]], input[2]) == after[input[3]] {acc += 1; vec.push("setr");}
            if self.set(input[1], input[2]) == after[input[3]] {acc += 1; vec.push("seti");}
            if input[1] < 4 && input[2] < 4 && self.gtr(before[input[1]], before[input[2]]) == after[input[3]] {acc += 1; vec.push("gtrr");}
            if input[1] < 4 && self.gtr(before[input[1]], input[2]) == after[input[3]] {acc += 1; vec.push("gtri");}
            if input[2] < 4 && self.gtr(input[1], before[input[2]]) == after[input[3]] {acc += 1; vec.push("gtir");}
            if input[1] < 4 && input[2] < 4 && self.eq(before[input[1]], before[input[2]]) == after[input[3]] {acc += 1; vec.push("eqrr");}
            if input[1] < 4 && self.eq(before[input[1]], input[2]) == after[input[3]] {acc += 1; vec.push("eqri");}
            if input[2] < 4 && self.eq(input[1], before[input[2]]) == after[input[3]] {acc += 1; vec.push("eqir");}
            if acc > 2 {result += 1} 
            // println!("op {}: {:?}", input[0], vec);
        }
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let mut register = vec![0, 0, 0, 0];
        for ins in &self.program {
            match self.op_map[ins[0]].as_str() {
                "addr" => register[ins[3]] = self.add(register[ins[1]], register[ins[2]]),
                "addi" => register[ins[3]] = self.add(register[ins[1]], ins[2]),
                "mulr" => register[ins[3]] = self.mul(register[ins[1]], register[ins[2]]),
                "muli" => register[ins[3]] = self.mul(register[ins[1]], ins[2]),
                "banr" => register[ins[3]] = self.ban(register[ins[1]], register[ins[2]]),
                "bani" => register[ins[3]] = self.ban(register[ins[1]], ins[2]),
                "borr" => register[ins[3]] = self.bor(register[ins[1]], register[ins[2]]),
                "bori" => register[ins[3]] = self.bor(register[ins[1]], ins[2]),
                "setr" => register[ins[3]] = self.set(register[ins[1]], ins[2]),
                "seti" => register[ins[3]] = self.set(ins[1], ins[2]),
                "gtrr" => register[ins[3]] = self.gtr(register[ins[1]], register[ins[2]]),
                "gtri" => register[ins[3]] = self.gtr(register[ins[1]], ins[2]),
                "gtir" => register[ins[3]] = self.gtr(ins[1], register[ins[2]]),
                "eqrr" => register[ins[3]] = self.eq(register[ins[1]], register[ins[2]]),
                "eqri" => register[ins[3]] = self.eq(register[ins[1]], ins[2]),
                "eqir" => register[ins[3]] = self.eq(ins[1], register[ins[2]]),
                _ => {}
            }
        }
        register[0].to_string()
    }
}
