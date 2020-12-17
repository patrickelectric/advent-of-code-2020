use aoc2020::helper;

mod map;
use map::Map;

fn main() {
    let mut map = Map::load_map("17-input.txt");
    for _ in 0..6 {
        map.run();
    }
    helper::print_answer("17-2", map.count_actives());
}
