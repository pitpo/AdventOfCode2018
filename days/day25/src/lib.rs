extern crate utils;

use utils::Day;

pub struct Day25 {
    input: Vec<Vec<isize>>,
}

impl Day25 {
    pub fn new(input: String) -> Day25 {
        let input = utils::extract_integers_from_string(&input);
        Day25 { input }
    }

    fn is_adjacent(&self, lhs: &Vec<isize>, rhs: &Vec<isize>) -> bool {
        (lhs[0] - rhs[0]).abs() + (lhs[1] - rhs[1]).abs() + (lhs[2] - rhs[2]).abs() + (lhs[3] - rhs[3]).abs() <= 3
    }
}

impl Day for Day25 {
    fn get_part_a_result(&self) -> String {
        let mut constellations: Vec<Vec<Vec<isize>>> = Vec::new();
        for coords in self.input.iter() {
            let mut found = false;
            for constellation in &mut constellations {
                let mut can_add = false;
                for other_coords in constellation.iter() {
                    if self.is_adjacent(&coords, other_coords) {
                        found = true;
                        can_add = true;
                    }
                }
                if can_add {
                    constellation.push(coords.clone());
                }
            }
            if !found {
                let mut constellation: Vec<Vec<isize>> = Vec::new();
                constellation.push(coords.clone());
                constellations.push(constellation);
            } else {
                let mut i = 0;
                while i < constellations.len() {
                    let mut j = i + 1;
                    while j < constellations.len() {
                        let mut join = false;
                        'outer: for coords in &constellations[i] {
                            for other_coords in &constellations[j] {
                                if self.is_adjacent(coords, other_coords) {
                                    join = true;
                                    break 'outer;
                                }
                            }
                        }
                        if join {
                            constellations[i] = constellations[i].clone().into_iter().chain(constellations[j].clone().into_iter()).collect::<Vec<Vec<isize>>>();
                            constellations.remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    i += 1;
                }
            }
        }
        constellations.len().to_string()
    }
    fn get_part_b_result(&self) -> String {
        String::from("Merry Christmas!")
    }
}
