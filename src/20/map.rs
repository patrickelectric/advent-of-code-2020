use regex::Regex;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u64,
    pub image: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Transform {
    pub flip: bool,
    pub rotate: u8, // 90º rotation number (0 = 0º, 1 = 90º, ..)
    pub side: u8,   // 0 = top, 1 = right, bottom = left, ..
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

    pub fn apply_transform(&mut self, transform: &Transform) -> &mut Self {
        if transform.flip {
            self.flip();
        }

        for _ in 0..transform.rotate {
            self.rotate();
        }

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
                    .filter(|(tile_side, (_, self_side))| tile_side == self_side)
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
        if self.map.values().any(|map_tile| map_tile.id == tile.id) {
            return false;
        }
        for (&(x, y), map_tile) in self.map.clone().iter() {
            match map_tile.matches(tile) {
                Some(transform) => {
                    let map_position = match transform.side {
                        0 => (x, y + 1),
                        1 => (x + 1, y),
                        2 => (x, y - 1),
                        3 => (x - 1, y),
                        _ => unreachable!(),
                    };
                    let mut tile = tile.clone();
                    tile.apply_transform(&transform);
                    if self.map.contains_key(&map_position) {
                        println!("what: {:#?}", map_position);
                        continue;
                    }
                    self.map.insert(map_position, tile.clone());
                    return true;
                }
                _ => continue,
            }
        }
        return false;
    }

    pub fn position(&self) -> ((i64, i64), (i64, i64)) {
        let keys = self.map.keys().map(|&x| x).collect::<Vec<(i64, i64)>>();
        let mut keys_x: Vec<i64> = keys.clone().iter().map(|value| value.0).collect();
        let mut keys_y: Vec<i64> = keys.clone().iter().map(|value| value.1).collect();
        keys_x.sort();
        keys_y.sort();
        let top_left = (*keys_x.first().unwrap(), *keys_y.last().unwrap());
        let bottom_right = (*keys_x.last().unwrap(), *keys_y.first().unwrap());

        return (top_left, bottom_right);
    }

    pub fn check_value(&self) -> u64 {
        let keys = self.map.keys().map(|&x| x).collect::<Vec<(i64, i64)>>();
        let mut keys_x: Vec<i64> = keys.clone().iter().map(|value| value.0).collect();
        let mut keys_y: Vec<i64> = keys.clone().iter().map(|value| value.1).collect();
        keys_x.sort();
        keys_y.sort();
        let top_left = (keys_x.first().unwrap(), keys_y.last().unwrap());
        let top_right = (keys_x.last().unwrap(), keys_y.last().unwrap());
        let bottom_left = (keys_x.first().unwrap(), keys_y.first().unwrap());
        let bottom_right = (keys_x.last().unwrap(), keys_y.first().unwrap());

        return vec![top_left, top_right, bottom_left, bottom_right]
            .iter()
            .map(|(&x, &y)| self.map.get(&(x, y)).unwrap().id)
            .collect::<Vec<u64>>()
            .iter()
            .product();
    }

    pub fn create_image(&self) -> Vec<String> {
        let position = self.position();
        let first_tile = self.map[&(0, 0)].clone();
        let height = ((position.0).1 - (position.1).1) as usize;
        let mut new_image: Vec<String> =
            vec![String::new(); (height + 1) * (first_tile.image.len() - 2)];

        for y in ((position.1).1..=(position.0).1).rev() {
            for x in (position.0).0..=(position.1).0 {
                let tile_image = &self.map[&(x, y)].image;
                for (index, line) in tile_image[1..tile_image.len() - 1].iter().enumerate() {
                    let y = y.abs();
                    let line = &line.to_string()[1..line.len() - 1];
                    let line_y_position = index + y as usize * (first_tile.image.len() - 2);
                    new_image[line_y_position] += line.into();
                }
            }
        }

        return new_image;
    }
}
