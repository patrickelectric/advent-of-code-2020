use aoc2020::helper;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    pub map: Vec<String>,
    nmap: Vec<String>,
    size: (u64, u64),
    pub steps: u64,
}

impl Map {
    pub fn new(filename: &str) -> Self {
        let content = helper::get_input_file(filename);
        let map: Vec<String> = content.lines().map(|line| line.to_string()).collect();
        let size = (map.first().unwrap().len() as u64, map.len() as u64); //0 = col, 1 = row

        return Self {
            map: map.clone(),
            nmap: map,
            size: size,
            steps: 0,
        };
    }

    fn set_char(&mut self, position: (u64, u64), character: char) {
        unsafe {
            self.nmap
                .get_mut(position.1 as usize)
                .unwrap()
                .as_bytes_mut()[position.0 as usize] = character as u8;
        }
    }

    fn get_char(&self, position: (u64, u64)) -> char {
        return self.map.get(position.1 as usize).unwrap().as_bytes()[position.0 as usize] as char;
    }

    fn get_char_sig(&self, position: (i64, i64)) -> char {
        if position.0 < 0
            || position.1 < 0
            || position.0 >= self.size.0 as i64
            || position.1 >= self.size.1 as i64
        {
            return '.';
        }
        return self.map.get(position.1 as usize).unwrap().as_bytes()[position.0 as usize] as char;
    }

    pub fn total_occuped_seats(&self) -> u64 {
        let mut result = 0;
        for line in self.map.iter() {
            result += line.chars().filter(|&x| x == '#').count() as u64;
        }
        return result;
    }

    pub fn count_neighbours(&self, position: (u64, u64)) -> u64 {
        let p = (position.0 as i64, position.1 as i64);
        let mut count = 0;
        for row in std::cmp::max(0, p.1 - 1)..std::cmp::min(self.size.1 as i64, p.1 + 2) {
            for column in std::cmp::max(0, p.0 - 1)..std::cmp::min(self.size.0 as i64, p.0 + 2) {
                if row == p.1 && column == p.0 {
                    continue;
                }
                count += (self.get_char((column as u64, row as u64)) == '#') as u64;
            }
        }
        return count;
    }

    pub fn run(&mut self) -> bool {
        self.steps += 1;
        self.nmap = self.map.clone();
        for row_index in 0..self.size.1 {
            for column_index in 0..self.size.0 {
                let position = (column_index, row_index);
                let count = self.count_neighbours(position);
                if count == 0 && self.get_char(position) == 'L' {
                    self.set_char(position, '#');
                } else if count >= 4 && self.get_char(position) == '#' {
                    self.set_char(position, 'L');
                }
            }
        }
        let result = self.map != self.nmap;
        self.map = self.nmap.clone();
        return result;
    }

    // part 2

    pub fn count_neighbours_2(&self, position: (u64, u64)) -> u64 {
        let p = (position.0 as i64, position.1 as i64);

        let mut hits = HashMap::new();
        'outside_loop: for radius in 1..std::cmp::max(self.size.0 as i64, self.size.1 as i64) {
            for &delta_y in [-1, 0, 1 as i64].iter() {
                for &delta_x in [-1, 0, 1 as i64].iter() {
                    let position = (p.0 + radius * delta_x, p.1 + radius * delta_y);
                    if position.0 == p.0 && position.1 == p.1 {
                        continue;
                    }
                    let hit = self.get_char_sig(position);
                    if !hits.contains_key(&(delta_x, delta_y)) && (hit == '#' || hit == 'L') {
                        hits.insert((delta_x, delta_y), hit);
                    }

                    if hits.len() == 8 {
                        break 'outside_loop;
                    }
                }
            }
        }

        return hits.values().filter(|&&x| x == '#').count() as u64;
    }

    pub fn run_2(&mut self) -> bool {
        self.steps += 1;
        self.nmap = self.map.clone();
        for row_index in 0..self.size.1 {
            for column_index in 0..self.size.0 {
                let position = (column_index, row_index);
                let count = self.count_neighbours_2(position);
                if count == 0 && self.get_char(position) == 'L' {
                    self.set_char(position, '#');
                } else if count >= 5 && self.get_char(position) == '#' {
                    self.set_char(position, 'L');
                }
            }
        }
        let result = self.map != self.nmap;
        self.map = self.nmap.clone();
        return result;
    }
}
