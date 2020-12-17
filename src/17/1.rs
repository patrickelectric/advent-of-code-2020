use aoc2020::helper;

mod map;
use map::Map;

fn main() {
    let mut map = Map::load_map("17-input.txt");
    map.change_to_three_dimensions();
    for _ in 0..6 {
        map.run();
    }
    helper::print_answer("17-1", map.count_actives());
}
