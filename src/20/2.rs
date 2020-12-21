use aoc2020::helper;

mod map;
use map::*;

use regex::Regex;

pub fn rotate(original_image: &Vec<String>) -> Vec<String> {
    let vec_chars: Vec<std::str::Chars<'_>> =
        original_image.iter().map(|line| line.chars()).collect();
    let mut image = vec![String::new(); original_image.len()];
    for chars in vec_chars {
        for (position, char_iter) in (0..image.len()).zip(chars) {
            *image.get_mut(position).unwrap() =
                char_iter.to_string() + image.get(position).unwrap();
        }
    }
    return image;
}

pub fn flip(original_image: &Vec<String>) -> Vec<String> {
    let image = original_image
        .iter()
        .rev()
        .map(|line| line.clone())
        .collect();
    return image;
}

pub fn find_monster(image: &Vec<String>) -> u64 {
    let monster_mask = include_str!("monster.txt")
        .lines()
        .map(|line| line.replace(" ", r"\S").into())
        .collect::<Vec<String>>();

    println!("{:#?}", monster_mask);
    let mut line_iter = image.iter();
    let mut mask_iter = monster_mask.iter();

    let mut dragons = 0;

    while let Some(line) = line_iter.next() {
        if let Some(mask_line) = mask_iter.next() {
            let re = Regex::new(mask_line).unwrap();
            if re.captures(line).is_some() {
                println!("> line: {}", line);
                continue;
            }
            mask_iter = monster_mask.iter();
        } else {
            dragons += 1;
            mask_iter = monster_mask.iter();
        }
    }

    return dragons;
}

fn main() {
    let content = helper::get_input_file("20-input.txt");
    let tiles: Vec<Tile> = content.split("\n\n").map(|tile| Tile::new(tile)).collect();
    let mut image = Image::new(&tiles.first().unwrap());
    loop {
        println!("image.len: {}/{}", image.map.len(), tiles.len());
        if image.map.len() == tiles.len() {
            break;
        }

        for tile in tiles.iter() {
            image.assembly(tile);
        }
    }

    let new_image = image.create_image();

    helper::print_answer("20-1", image.check_value());
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

        let mut image = Image::new(&tile_1951.clone().flip());
        image.assembly(&tile_2311);

        loop {
            if image.map.len() == tiles.len() {
                break;
            }

            for tile in tiles.iter() {
                image.assembly(tile);
            }
        }
        assert_eq!(image.check_value(), 20899048083289);

        let final_map = image.create_image();
        assert_eq!(final_map.join("\n"), include_str!("example-no-gaps.txt"));

        assert_eq!(find_monster(&rotate(&flip(&final_map))), 2);
    }
}
