use aoc2020::helper;

use std::collections::HashMap;

fn main() {
    let notes = helper::get_input_file("13-input.txt");
    let mut notes = notes.lines();
    let _actual_time = notes.next().unwrap().parse::<u64>().unwrap();
    let note_ids: HashMap<u64, Result<u64, _>> = notes
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .map(|(index, value)| (index as u64, value.parse::<u64>()))
        .collect();
    let id_to_delta: HashMap<u64, u64> = note_ids
        .iter()
        .filter(|&(_, result)| result.is_ok())
        .map(|(&index, result)| (result.clone().unwrap(), index))
        .collect();

    let mut ids = id_to_delta.keys().copied().collect::<Vec<u64>>();
    ids.sort();

    let &min_id = ids.first().unwrap();
    let mut pivot = min_id; // initial value
    let mut time = 0;
    'outside_loop: loop {
        time += pivot;

        for (index, id) in ids.iter().enumerate() {
            if (time + id_to_delta[&id]) % id != 0 {
                // the latest n matching will happen again, as the prophecy says.
                pivot = ids[0..index].iter().product();
                continue 'outside_loop;
            }
        }
        break;
    }

    helper::print_answer("13-2", time);
}
