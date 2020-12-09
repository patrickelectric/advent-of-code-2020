use aoc2020::helper;

fn has_sum(values: &[u64], target: u64) -> bool {
    for value1 in values.iter() {
        for value2 in values.iter() {
            if value1 != value2 && value1 + value2 == target {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let preamble = 25;
    let values = helper::get_numbers_from_file("9-input.txt");

    for (index, value) in values.iter().skip(preamble).enumerate() {
        if !has_sum(&values[index..index + preamble], *value) {
            helper::print_answer("9-1", *value);
            break;
        }
    }
}
