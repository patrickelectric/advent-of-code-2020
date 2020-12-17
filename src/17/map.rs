use aoc2020::helper;

use std::collections::HashMap;

#[derive(Default, Debug, Eq, Hash, Clone, PartialEq, PartialOrd, Ord)]
pub struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

#[derive(Debug)]
pub struct Map {
    map: HashMap<Coordinate, bool>,
    dimensions: u64,
}

impl Map {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
            dimensions: 4,
        };
    }

    pub fn change_to_three_dimensions(&mut self) {
        self.dimensions = 3;
    }

    pub fn load_map(filename: &str) -> Self {
        let content = helper::get_input_file(filename);
        let mut map = Map::new();

        for (y, line) in content.lines().enumerate() {
            let mut coordinate: Coordinate = Default::default();
            coordinate.w = 0;
            coordinate.z = 0;
            coordinate.y = y as i64;
            for (x, c) in line.chars().enumerate() {
                coordinate.x = x as i64;
                map.map.insert(coordinate.clone(), c == '#');
            }
        }

        return map;
    }

    // move to trait debug
    pub fn print(&self) {
        let mut keys: Vec<Coordinate> = self.map.keys().cloned().collect();
        keys.sort();
        let first_layer = keys.first().unwrap();
        let last_layer = keys.last().unwrap();
        for w in first_layer.w..=last_layer.w {
            for z in first_layer.z..=last_layer.z {
                println!("-------\nz: {}", z);
                for y in first_layer.y..=last_layer.y {
                    for x in first_layer.x..=last_layer.x {
                        print!("{}", if self.map[&Coordinate{x, y, z, w}] {'#'} else {'.'});
                    }
                    println!();
                }
            }
        }
    }

    pub fn count_neighbours(&self, coordinate: &Coordinate) -> u64 {
        let mut total = 0;
        for w in coordinate.w-1..=coordinate.w+1 {
            for z in coordinate.z-1..=coordinate.z+1 {
                for y in coordinate.y-1..=coordinate.y+1 {
                    for x in coordinate.x-1..=coordinate.x+1 {
                        if self.dimensions == 3 && w != 0 {
                            continue
                        }
                        let local_coordinate = Coordinate{x, y, z, w};
                        if local_coordinate == *coordinate {
                            continue;
                        }
                        if *self.map.get(&local_coordinate).unwrap_or(&false) {
                            total += 1;
                        }
                    }
                }
            }
        }
        return total;
    }

    pub fn run(&mut self) {
        let mut map = self.map.clone();

        let mut keys: Vec<Coordinate> = self.map.keys().cloned().collect();
        keys.sort();

        let first_layer = keys.first().unwrap();
        let last_layer = keys.last().unwrap();

        for w in first_layer.w-1..=last_layer.w+1 {
            for z in first_layer.z-1..=last_layer.z+1 {
                for y in first_layer.y-1..=last_layer.y+1 {
                    for x in first_layer.x-1..=last_layer.x+1 {
                        if self.dimensions == 3 && w != 0 {
                            continue
                        }

                        let coordinate = Coordinate{x, y, z, w};
                        let active = *self.map.get(&coordinate).unwrap_or(&false);
                        let neighbours = self.count_neighbours(&Coordinate{x, y, z, w});

                        // add if does not exist
                        map.insert(coordinate.clone(), active);

                        if !active && neighbours == 3 {
                            map.insert(coordinate, true);
                            continue;
                        }

                        if active && (neighbours == 2 || neighbours == 3) {
                            continue;
                        } else {
                            map.insert(coordinate.clone(), false);
                        }
                    }
                }
            }
        }
        self.map = map;
    }

    pub fn count_actives(&self) -> u64 {
        return self.map.values().filter(|&&value| value).count() as u64;
    }
}