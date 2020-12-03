use aoc2020::helper;

mod map;

fn main() {
    let map_string = helper::get_input_file("3-input.txt");
    let mut threes = map::get_three_hits(&map_string, &(1, 1));

    for step in [(1, 3), (1, 5), (1, 7), (2, 1)].iter() {
        threes *= map::get_three_hits(&map_string, step);
    }

    helper::print_answer("3-2", threes);
}
