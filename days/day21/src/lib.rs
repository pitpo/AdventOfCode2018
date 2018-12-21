extern crate utils;

use utils::Day;
use std::collections::BTreeSet;

pub struct Day21 {

}

impl Day21 {
    pub fn new(input: String) -> Day21 {
        Day21 { }
    }
}

impl Day for Day21 {
    fn get_part_a_result(&self) -> String {
        let (mut b, mut c, mut d, mut f) = (0, 0, 0, 0);
        f = c | 65536;
        c = 2238642;
        loop {
            d = f & 255;
            c = d + c;
            c = c & 16777215;
            c = c * 65899;
            c = c & 16777215;
            if 256 > f {
                return c.to_string();
            } else {
                d = 0;
                loop {
                    b = d + 1;
                    b = b * 256;
                    if b > f {
                        f = d;
                        break;
                    } else {
                        d = d + 1;
                    }
                }
            }
        }
    }
    fn get_part_b_result(&self) -> String {
        let (mut b, mut c, mut d, mut f, mut last) = (0, 0, 0, 0, 0);
        let mut set: BTreeSet<usize> = BTreeSet::new();
        'outer: loop {
            f = c | 65536;
            c = 2238642;
            loop {
                d = f & 255;
                c = d + c;
                c = c & 16777215;
                c = c * 65899;
                c = c & 16777215;
                if 256 > f {
                    if !set.contains(&c) {
                        last = c
                    } else {
                        return last.to_string();
                    }
                    set.insert(c);
                    continue 'outer;
                } else {
                    d = 0;
                    loop {
                        b = d + 1;
                        b = b * 256;
                        if b > f {
                            f = d;
                            break;
                        } else {
                            d = d + 1;
                        }
                    }
                }
            }
        }
    }
}
