use aoc2020::helper;

/**
 * Do the simple logic, populate vector, remove the last number and do the math
 */
fn main() {
    let starting_numbers = helper::get_input_file("15-input.txt");
    let starting_numbers: Vec<u64> = starting_numbers
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let numbers = starting_numbers;

    let get_number = |numbers: &Vec<u64>, position: usize| -> u64 {
        let mut numbers = numbers.clone();
        while numbers.len() != position {
            let &last_number = numbers.last().unwrap();
            let value = match numbers[0..numbers.len() - 1]
                .iter()
                .rposition(|&x| x == last_number)
            {
                None => 0,
                Some(x) => numbers.len() - x - 1,
            };
            numbers.push(value as u64);
        }
        return *numbers.last().unwrap();
    };

    helper::print_answer("15-1", get_number(&numbers, 2020));
}
