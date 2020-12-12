use aoc2020::helper;

mod map;

fn main() {
    let mut map = map::Map::new("11-input.txt");
    while map.run() {}
    helper::print_answer("11-1", map.total_occuped_seats());
}
