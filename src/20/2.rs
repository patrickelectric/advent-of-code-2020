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

    let mut monsters: u64 = 0;
    let original_image = image.create_image();

    'outside: for flip_image in [false, true].iter() {
        let image = original_image.clone();
        let mut image = if *flip_image { flip(&image) } else { image };
        for rotate_amount in 0..4 {
            if rotate_amount != 0 {
                image = rotate(&image);
            }
            //println!("{:#?}", image);
            let amount = find_monster(&image);
            println!("flip: {}, rotate: {}, momsters: {}", flip_image, rotate_amount, amount);
            monsters += amount;
            /*
            if monsters != 0 {
                break 'outside;
            }*/
        }
    }

    //2806
    let waves: u64 = original_image.iter().map(|line| line.chars().filter(|character| character == &'#').count() as u64).sum::<u64>() - monsters * 15;
    println!("monsters: {}", monsters);

    helper::print_answer("20-2", waves); // high 20806, high 2806
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
