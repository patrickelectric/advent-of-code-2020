use aoc2020::helper;

use regex::Regex;

#[derive(Debug)]
pub struct Tile {
    pub id: u64,
    image: Vec<String>,
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
        println!("tiles: {:#?}", tiles);
    }
}
