use aoc2020::helper;

use std::collections::HashMap;
use std::ops::{Add, AddAssign};

#[derive(Default, Debug, Eq, Hash, Copy, Clone, PartialEq, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Map {
    tiles: HashMap<Coordinate, bool>,
    position: Coordinate,
}

impl Map {
    pub fn new() -> Self {
        return Self {
            tiles: HashMap::new(),
            position: Coordinate { x: 0, y: 0 },
        };
    }

    pub fn count_state(&self, state: bool, position: &Coordinate) -> u64 {
        let mut count = 0;
        for &(x, y) in [(-1,1), (0,1), (1,1), (1,0), (-1,-1), (0,-1), (-1,0)].iter() {
            let coordinate = *position + Coordinate{x, y};
            match self.tiles.get(&coordinate) {
                Some(&value) => count += value as u64,
                _ => continue,
            }
        }
        //println!("count {}", count);
        if state {
            return count;
        }

        return 6 - count;
    }

    pub fn update_move(&mut self, x: i64, y: i64) {
        self.position.x += x;
        self.position.y += y;
    }

    pub fn process(&mut self, string: &str) -> &mut Map {
        let mut previous_c: Option<char> = None;
        self.position.x = 0;
        self.position.y = 0;
        self.update_move(0, 0);
        for c in string.chars() {
            match c {
                'e' => {
                    match previous_c {
                        Some('n') => {
                            self.update_move(0, 1);
                        }
                        Some('s') => {
                            self.update_move(1, -1);
                        }
                        _ => {
                            self.update_move(1, 0);
                        }
                    }
                    previous_c = None;
                }
                'w' => {
                    match previous_c {
                        Some('n') => {
                            self.update_move(-1, 1);
                        }
                        Some('s') => {
                            self.update_move(0, -1);
                        }
                        _ => {
                            self.update_move(-1, 0);
                        }
                    }
                    previous_c = None;
                }
                's' | 'n' => {
                    previous_c = Some(c);
                }
                _ => {
                    unreachable!("impossible position.");
                }
            }
        }

        if self.tiles.contains_key(&self.position) {
            let new_value = !self.tiles[&self.position];
            *self.tiles.get_mut(&self.position).unwrap() = new_value;
        } else {
            self.tiles.insert(self.position, true);
        }

        return self;
    }

    pub fn finish_day(&mut self) {
        for _ in 0..1 {
            for coordinate in self.tiles.clone().keys().cloned() {
                for &(x, y) in [(-1,1), (0,1), (1,1), (1,0), (-1,-1), (0,-1), (-1,0)].iter() {
                    let coordinate = coordinate + Coordinate{x, y};
                    if !self.tiles.contains_key(&coordinate) {
                        self.tiles.insert(coordinate, false);
                    }
                }
            }
        }

        let mut tiles = self.tiles.clone();

        for coordinate in self.tiles.keys().cloned() {
            let tile = tiles.get_mut(&coordinate).unwrap();
            //println!("------\ntile i: {}", tile);
            if *self.tiles.get(&coordinate).unwrap() {
                match self.count_state(true, &coordinate) {
                    0|3..=6 => *tile = false,
                    _ => {},
                }
            } else {
                match self.count_state(true, &coordinate) {
                    2 => *tile = true,
                    _ => {},
                }
            }
            //println!("tile o: {}", tile);

            /*
            for &(x, y) in [(-1,1), (0,1), (1,1), (1,0), (-1,-1), (0,-1), (-1,0)].iter() {
                let coordinate = coordinate + Coordinate{x, y};
                if !tiles.contains_key(&coordinate) {
                    tiles.insert(coordinate, false);
                }
            }*/
        }

        self.tiles = tiles;
    }

    pub fn count(&self) -> u64 {
        return self.tiles.values().filter(|&&val| val).count() as u64;
    }
}

fn main() {
    let content = helper::get_input_file("24-input.txt");
    let mut map = Map::new();
    for line in content.lines() {
        map.process(line);
    }
    helper::print_answer("24-2", map.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut map = Map::new();
        for _ in 0..1 {
            for line in include_str!("example.txt").lines() {
                map.process(line);
            }
            map.finish_day();
        }
        assert_eq!(map.count(), 15);
    }
}
