use aoc2020::helper;

use std::collections::HashMap;
use std::ops::AddAssign;

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
    helper::print_answer("24-1", map.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut map = Map::new();
        for line in include_str!("example.txt").lines() {
            map.process(line);
        }
        assert_eq!(map.count(), 10);
    }
}
