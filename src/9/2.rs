use aoc2020::helper;

fn main() {
    let invalid_number = 1212510616;
    let values = helper::get_numbers_from_file("9-input.txt");

    'outside_loop: for test_index in 0..values.len() {
        let mut answer = vec![];
        for value in values.iter().skip(test_index) {
            answer.push(*value);
            let accumulator: u64 = answer.iter().sum();

            if accumulator > invalid_number {
                break;
            }

            if accumulator == invalid_number {
                answer.sort();
                let encryption_weakness = answer.first().unwrap() + answer.last().unwrap();
                helper::print_answer("9-2", encryption_weakness);
                break 'outside_loop;
            }
        }
    }
}
