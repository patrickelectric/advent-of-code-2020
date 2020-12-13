use aoc2020::helper;

fn main() {
    let mut adapters = helper::get_numbers_from_file("10-input.txt");
    adapters.push(0); // outlet
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3); // joltage adapter

    let arranged: Vec<u64> = adapters
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect();

    let answer = arranged.iter().filter(|&&x| x == 1 as u64).count()
        * arranged.iter().filter(|&&x| x == 3 as u64).count();

    helper::print_answer("10-1", answer as u64)
}
