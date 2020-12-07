use aoc2020::helper;

use std::collections::HashSet;

fn main() {
    let forms_table = helper::get_input_file("6-input.txt");
    let mut answer = 0;
    for line in forms_table.split("\n\n") {
        let lines: Vec<HashSet<char>> = line
            .lines()
            .map(|string| string.chars().collect::<HashSet<char>>())
            .collect();
        let lines: HashSet<char> = lines.iter().fold(lines.first().unwrap().clone(), |x, y| {
            x.intersection(y).map(|x| *x).collect::<HashSet<char>>()
        });
        answer += lines.len() as u64;
    }

    helper::print_answer("6-2", answer);
}
