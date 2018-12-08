extern crate utils;

use utils::Day;

pub struct Day8 {
    input: String,
}

impl Day8 {
    pub fn new(input: String) -> Day8 {
        Day8 { input }
    }

    fn parse_input(&self) -> Vec<usize> {
        utils::extract_unsigned_integers_from_string(&self.input)
            .get(0)
            .unwrap()
            .clone()
    }

    fn traverse_tree_a(&self, tree: &Vec<usize>, first_index: usize) -> (usize, usize) {
        let subtrees = tree[first_index];
        let values = tree[first_index + 1];
        let mut result = 0;
        let mut next_first = first_index + 2;
        for _ in 0..subtrees {
            let (sum, next) = self.traverse_tree_a(tree, next_first);
            next_first = next;
            result += sum;
        }
        for i in 0..values {
            result += tree[next_first + i]
        }
        return (result, next_first + values);
    }

    fn traverse_tree_b(&self, tree: &Vec<usize>, first_index: usize) -> (usize, usize) {
        let subtrees = tree[first_index];
        let values = tree[first_index + 1];
        let mut result = 0;
        let mut next_first = first_index + 2;
        let mut children_sums: Vec<usize> = Vec::new();
        for _ in 0..subtrees {
            let (sum, next) = self.traverse_tree_b(tree, next_first);
            next_first = next;
            children_sums.push(sum);
        }
        if subtrees == 0 {
            for i in 0..values {
                result += tree[next_first + i]
            }
        } else {
            for i in 0..values {
                let value = tree[next_first + i];
                if value <= subtrees {
                    result += children_sums[value - 1];
                }
            }
        }
        return (result, next_first + values);
    }
}

impl Day for Day8 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let result = self.traverse_tree_a(&input, 0).0;
        result.to_string()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let result = self.traverse_tree_b(&input, 0).0;
        result.to_string()
    }
}
