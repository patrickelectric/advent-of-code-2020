use aoc2020::helper;

use std::collections::HashMap;

/**
 * Hi future me,
 *  I hope you are doing well, right now we are facing a pandemic but I should be fine
 *  if my future version is reading this.
 * Ok, now about the logic: Get actual time, check the rest of division with valid ids,
 * get the (id - rest) this will show how long you'll wait, sort that and get the id back.
 */
fn main() {
    let notes = helper::get_input_file("13-input.txt");
    let mut notes = notes.lines();
    let actual_time = notes.next().unwrap().parse::<u64>().unwrap();
    let mut note_ids: Vec<u64> = notes
        .next()
        .unwrap()
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    note_ids.sort();

    let delta_map: HashMap<u64, u64> = note_ids.iter().map(|&x| (x - actual_time % x, x)).collect();
    let mut delta_to_ids = delta_map.keys().copied().collect::<Vec<u64>>();
    delta_to_ids.sort();

    let &next_bus_delta = delta_to_ids.first().unwrap();
    helper::print_answer("13-1", next_bus_delta * delta_map[&next_bus_delta]);
}
