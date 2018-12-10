extern crate utils;

use std::collections::VecDeque;
use utils::Day;

pub struct Day9 {
    input: String,
}

impl Day9 {
    pub fn new(input: String) -> Day9 {
        Day9 { input }
    }

    fn parse_input(&self) -> Vec<usize> {
        utils::extract_unsigned_integers_from_string(&self.input)
            .get(0)
            .unwrap()
            .clone()
    }

    fn rotate(&self, arr: &mut VecDeque<usize>, rotation: i8) {
        for _ in 0..rotation.abs() {
            if rotation < 0 {
                let val = arr.pop_back().unwrap();
                arr.push_front(val);
            } else {
                let val = arr.pop_front().unwrap();
                arr.push_back(val);
            }
        }
    }

    fn get_max_score(&self, num_of_players: usize, last_marble: usize) -> usize {
        // coming up with this solution took so long, I'm sure the bruteforce that was running in the background was almost done
        let mut marbles: VecDeque<usize> = VecDeque::with_capacity(last_marble);
        marbles.push_back(0);
        let mut player_scores: Vec<usize> = Vec::new();
        for _ in 0..num_of_players {
            player_scores.push(0);
        }
        let mut current_player = 1;
        for marble in 1..last_marble {
            if marble % 23 != 0 {
                self.rotate(&mut marbles, 2);
                marbles.push_front(marble);
            } else {
                self.rotate(&mut marbles, -7);
                player_scores[current_player] += marbles.pop_front().unwrap() + marble;
            }
            current_player = (current_player + 1) % num_of_players;
        }
        player_scores.into_iter().max().unwrap()
    }
}

impl Day for Day9 {
    fn get_part_a_result(&self) -> String {
        let input = self.parse_input();
        let (num_of_players, last_marble) = (input[0], input[1] + 1);
        self.get_max_score(num_of_players, last_marble).to_string()
    }
    fn get_part_b_result(&self) -> String {
        let input = self.parse_input();
        let (num_of_players, last_marble) = (input[0], input[1] * 100);
        self.get_max_score(num_of_players, last_marble).to_string()
    }
}
