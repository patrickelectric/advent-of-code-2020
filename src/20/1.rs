use aoc2020::helper;

mod map;
use map::*;

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
        let tile_1171 = tiles_iter.next().unwrap().clone();
        let _tile_1427 = tiles_iter.next().unwrap().clone();
        let tile_1489 = tiles_iter.next().unwrap().clone();
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

        assert_eq!(
            tile_2311.clone().matches(&tile_1951),
            Some(Transform {
                flip: false,
                rotate: 0,
                side: 3
            })
        );
        assert_eq!(
            tile_1951.clone().flip().matches(&tile_2311),
            Some(Transform {
                flip: true,
                rotate: 0,
                side: 1
            })
        );
        assert_eq!(
            tile_1489.clone().flip().matches(&tile_1171),
            Some(Transform {
                flip: true,
                rotate: 2,
                side: 1
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
    }
}
