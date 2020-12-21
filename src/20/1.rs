use aoc2020::helper;

use regex::Regex;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u64,
    image: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Transform {
    flip: bool,
    rotate: u8, // 90º rotation number (0 = 0º, 1 = 90º, ..)
    side: u8,   // 0 = top, 1 = right, bottom = left, ..
}

impl Tile {
    pub fn new(tile: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let mut lines = tile.lines();
        let tile_header = lines.next().unwrap();
        let id: u64 = re
            .captures(tile_header)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();

        let image = lines.map(|line| line.into()).collect();

        return Self { id, image };
    }

    pub fn sides(&self) -> Vec<String> {
        let top = self.image.first().unwrap().clone();
        let bottom = self.image.last().unwrap().clone();
        let left: String = self
            .image
            .iter()
            .map(|line| line.chars().next().unwrap())
            .collect();
        let right: String = self
            .image
            .iter()
            .map(|line| line.chars().rev().next().unwrap())
            .collect();

        return vec![top, right, bottom, left];
    }

    pub fn rotate(&mut self) -> &mut Self {
        let vec_chars: Vec<std::str::Chars<'_>> =
            self.image.iter().map(|line| line.chars()).collect();
        let mut image = vec![String::new(); self.image.len()];
        for chars in vec_chars {
            for (position, char_iter) in (0..image.len()).zip(chars) {
                *image.get_mut(position).unwrap() =
                    char_iter.to_string() + image.get(position).unwrap();
            }
        }

        self.image = image;

        return self;
    }

    pub fn flip(&mut self) -> &mut Self {
        self.image = self.image.iter().rev().map(|line| line.into()).collect();
        return self;
    }

    pub fn matches(&self, tile: &Tile) -> Option<Transform> {
        for &flip in [false, true].iter() {
            let mut tile = tile.clone();
            if flip {
                tile.flip();
            }

            for rotate in 0..4 {
                let mut tile_sides = tile.sides();
                if rotate != 0 {
                    tile_sides = tile.rotate().sides();
                }
                tile_sides.swap(0, 2);
                tile_sides.swap(1, 3);
                let side: Vec<usize> = tile_sides
                    .iter()
                    .zip(self.sides().iter().enumerate())
                    .filter(|(tile_side, (side, self_side))| tile_side == self_side)
                    .map(|(_, (side, _))| side)
                    .collect();
                if side.len() != 0 {
                    let side = *side.first().unwrap() as u8;
                    return Some(Transform { flip, rotate, side });
                }
            }
        }
        return None;
    }
}

#[derive(Debug)]
pub struct Image {
    pub map: HashMap<(i64, i64), Tile>,
}

impl Image {
    pub fn new(tile: &Tile) -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert((0, 0), tile.clone());
        return Image { map: hash_map };
    }

    pub fn assembly(&mut self, tile: &Tile) -> bool {
        for (&(x, y), map_tile) in self.map.clone().iter() {
            match tile.matches(map_tile) {
                Some(transform) => {
                    println!("{:#?}", transform);
                    let map_position = match transform.side {
                        0 => (x, y - 1),
                        1 => (x - 1, y),
                        2 => (x, y + 1),
                        3 => (x + 1, y),
                        _ => unreachable!(),
                    };
                    let mut tile = tile.clone();
                    if transform.flip {
                        tile.flip();
                    }
                    for _ in 0..transform.rotate {
                        tile.rotate();
                    }
                    self.map.insert(map_position, tile.clone());
                }
                _ => continue,
            }
        }
        return false;
    }

    pub fn check_value(&self) -> u64 {
        println!("{:#?}", self.map.keys());
        return 0;
    }
}

fn main() {
    let content = helper::get_input_file("20-input.txt");
    helper::print_answer("20-1", 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        let tiles: Vec<Tile> = input.split("\n\n").map(|tile| Tile::new(tile)).collect();
        assert_eq!(tiles.len(), 9);

        let mut tiles_iter = tiles.iter();
        let tile_2311 = tiles_iter.next().unwrap().clone();
        let tile_1951 = tiles_iter.next().unwrap().clone();
        assert_eq!(tile_2311.id, 2311);
        assert_eq!(
            tile_2311.sides(),
            vec!["..##.#..#.", "...#.##..#", "..###..###", ".#####..#."]
        );
        assert_eq!(
            tile_2311.clone().rotate().sides(),
            vec![".#..#####.", "..##.#..#.", "#..##.#...", "..###..###"]
        );
        assert_eq!(
            tile_2311.clone().flip().sides(),
            vec!["..###..###", "#..##.#...", "..##.#..#.", ".#..#####."]
        );
        println!("{:#?}", tile_2311);
        println!("{:#?}", tile_1951);
        assert_eq!(
            tile_2311.clone().matches(&tile_1951),
            Some(Transform {
                flip: false,
                rotate: 0,
                side: 3
            })
        );
        assert_eq!(
            tile_1951.clone().matches(&tile_2311),
            Some(Transform {
                flip: false,
                rotate: 0,
                side: 1
            })
        );

        let mut image = Image::new(&tile_1951);
        image.assembly(&tile_2311);
        println!("{:#?}", image);

        loop {
            if image.map.len() == tiles.len() {
                break;
            }

            for tile in tiles.iter() {
                image.assembly(tile);
            }
        }
        println!("{:#?}", image);
        image.check_value();
    }
}
