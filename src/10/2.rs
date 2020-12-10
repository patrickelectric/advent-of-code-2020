use aoc2020::helper;

use std::collections::HashMap;

fn find_branches(key: u64, map: &HashMap<u64, Vec<u64>>, temp: &mut HashMap<u64, u64>) -> u64 {
    let mut count = 0;
    if temp.contains_key(&key) {
        return temp[&key];
    }
    if map.contains_key(&key) {
        for &value in map[&key].iter() {
            count += find_branches(value, map, temp);
        }
    } else {
        count = 1;
    }
    temp.insert(key, count);
    return count;
}

fn main() {
    let mut adapters = helper::get_numbers_from_file("10-input.txt");
    adapters.push(0); // outlet
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3); // joltage adapter

    let valid_fun = |slice: &[u64]| -> Vec<u64> {
        if slice[3] - slice[0] <= 3 {
            return slice[1..slice.len()].to_vec();
        }
        if slice[2] - slice[0] <= 3 {
            return slice[1..slice.len() - 1].to_vec();
        }
        if slice[1] - slice[0] <= 3 {
            return slice[1..slice.len() - 2].to_vec();
        }
        unreachable!("wth");
    };

    let layers: HashMap<u64, Vec<u64>> = adapters
        .windows(4)
        .map(|window| (window[0], valid_fun(&window)))
        .collect();
    let mut temp: HashMap<u64, u64> = HashMap::new();
    helper::print_answer("10-2", find_branches(0, &layers, &mut temp))
}
