extern crate utils;

use std::cmp::Ordering;
use utils::Day;

#[derive(Copy, Debug, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        *self
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        match *self {
            Direction::North => if let Direction::North = *other {
                true
            } else {
                false
            },
            Direction::East => if let Direction::East = *other {
                true
            } else {
                false
            },
            Direction::South => if let Direction::South = *other {
                true
            } else {
                false
            },
            Direction::West => if let Direction::West = *other {
                true
            } else {
                false
            },
        }
    }
}

pub struct Day13 {
    map: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

#[derive(Copy, Debug, Eq)]
struct Cart {
    y: usize,
    x: usize,
    dir: Direction,
    turn: usize,
}

impl Clone for Cart {
    fn clone(&self) -> Cart {
        *self
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.x == other.x && self.y == other.y && self.turn == other.turn
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        let order = self.y.cmp(&other.y);
        if order == Ordering::Equal {
            return self.x.cmp(&other.x);
        }
        order
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day13 {
    pub fn new(input: String) -> Day13 {
        let map = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut carts: Vec<Cart> = Vec::new();
        for (i, y) in map.iter().enumerate() {
            for (j, x) in y.iter().enumerate() {
                match *x {
                    '^' => carts.push(Cart {
                        y: i,
                        x: j,
                        dir: Direction::North,
                        turn: 0,
                    }),
                    'v' => carts.push(Cart {
                        y: i,
                        x: j,
                        dir: Direction::South,
                        turn: 0,
                    }),
                    '>' => carts.push(Cart {
                        y: i,
                        x: j,
                        dir: Direction::East,
                        turn: 0,
                    }),
                    '<' => carts.push(Cart {
                        y: i,
                        x: j,
                        dir: Direction::West,
                        turn: 0,
                    }),
                    _ => {}
                }
            }
        }
        Day13 { map, carts }
    }

    fn go_straight(&self, cart: &mut Cart) -> Cart {
        match cart.dir {
            Direction::North => {
                cart.y -= 1;
            }
            Direction::South => {
                cart.y += 1;
            }
            Direction::East => {
                cart.x += 1;
            }
            Direction::West => {
                cart.x -= 1;
            }
        }
        *cart
    }

    fn go_left(&self, cart: &mut Cart) -> Cart {
        match cart.dir {
            Direction::North => {
                cart.x -= 1;
                cart.dir = Direction::West;
            }
            Direction::East => {
                cart.y -= 1;
                cart.dir = Direction::North;
            }
            Direction::South => {
                cart.x += 1;
                cart.dir = Direction::East;
            }
            Direction::West => {
                cart.y += 1;
                cart.dir = Direction::South;
            }
        }
        *cart
    }

    fn go_right(&self, cart: &mut Cart) -> Cart {
        match cart.dir {
            Direction::North => {
                cart.x += 1;
                cart.dir = Direction::East;
            }
            Direction::East => {
                cart.y += 1;
                cart.dir = Direction::South;
            }
            Direction::South => {
                cart.x -= 1;
                cart.dir = Direction::West;
            }
            Direction::West => {
                cart.y -= 1;
                cart.dir = Direction::North;
            }
        }
        *cart
    }

    fn crossroads(&self, cart: &mut Cart) -> Cart {
        *cart = match cart.turn % 3 {
            0 => self.go_left(cart),
            1 => self.go_straight(cart),
            2 => self.go_right(cart),
            _ => panic!("unreachable code"),
        };
        cart.turn += 1;
        *cart
    }

    fn make_move(&self, cart: &mut Cart) -> Cart {
        match cart.dir {
            Direction::North | Direction::South => match self.map[cart.y][cart.x] {
                '\\' => self.go_left(cart),
                '/' => self.go_right(cart),
                '+' => self.crossroads(cart),
                _ => self.go_straight(cart),
            },
            Direction::West | Direction::East => match self.map[cart.y][cart.x] {
                '\\' => self.go_right(cart),
                '/' => self.go_left(cart),
                '+' => self.crossroads(cart),
                _ => self.go_straight(cart),
            },
        }
    }

    fn tick(&self, carts: &mut Vec<Cart>) -> Vec<Cart> {
        carts.sort();
        let mut crashes: Vec<usize> = Vec::new();
        for cart in 0..carts.len() {
            self.make_move(&mut carts[cart]);
            for other_cart in 0..carts.len() {
                if cart != other_cart
                    && carts[cart].dir != carts[other_cart].dir
                    && carts[cart].x == carts[other_cart].x
                    && carts[cart].y == carts[other_cart].y
                {
                    crashes.push(cart);
                    crashes.push(other_cart);
                }
            }
        }
        crashes
            .into_iter()
            .map(|cart| carts[cart])
            .collect::<Vec<Cart>>()
    }
}

impl Day for Day13 {
    fn get_part_a_result(&self) -> String {
        let mut carts: Vec<Cart> = self.carts.to_vec();
        loop {
            let crashes = self.tick(&mut carts);
            match crashes.into_iter().min_by(|c1, c2| c1.y.cmp(&c2.y)) {
                Some(crash) => return format!("{},{}", crash.x, crash.y),
                None => {}
            }
        }
    }
    fn get_part_b_result(&self) -> String {
        let mut carts: Vec<Cart> = self.carts.to_vec();
        loop {
            let crashes = self.tick(&mut carts);
            for crash in &crashes {
                for i in 0..carts.len() {
                    if *crash == carts[i] {
                        carts.remove(i);
                        break;
                    }
                }
            }
            if carts.len() == 1 {
                return format!("{},{}", carts[0].x, carts[0].y);
            }
        }
    }
}
