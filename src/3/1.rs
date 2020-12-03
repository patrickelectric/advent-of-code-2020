use aoc2020::helper;

mod map;

fn main() {
    let map_string = helper::get_input_file("3-input.txt");
    let threes = map::get_three_hits(&map_string, &(1, 3));

    helper::print_answer("3-1", threes);
}
