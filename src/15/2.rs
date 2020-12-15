use aoc2020::helper;

use std::collections::HashMap;

/**
 * Create a map of value,position to improve speed
 */
fn main() {
    let starting_numbers = helper::get_input_file("15-input.txt");
    let starting_numbers: Vec<u64> = starting_numbers
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let numbers = starting_numbers;

    let get_number = |numbers: &Vec<u64>, position: u64| -> u64 {
        let mut map: HashMap<u64, u64> = numbers
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .map(|(index, &value)| (value, index as u64))
            .collect();
        let mut last_map_number = (*numbers.last().unwrap(), numbers.len() as u64 - 1);

        let mut size = numbers.len() as u64;
        while size != position {
            let last_number = last_map_number.0;
            if size % 10000 == 0 {
                print!("\r{}%", size * 100 / position);
            }
            if map.contains_key(&last_number) {
                let next_number = size - map[&last_number] - 1;
                map.insert(last_map_number.0, last_map_number.1);
                last_map_number = (next_number, size);
                size += 1;
                continue;
            }

            map.insert(last_map_number.0, last_map_number.1);
            last_map_number = (0, size);
            size += 1;
        }
        if position > 10000 {
            println!();
        }
        return last_map_number.0;
    };

    helper::print_answer("15-1", get_number(&numbers, 2020));
    helper::print_answer("15-2", get_number(&numbers, 30000000));
}
