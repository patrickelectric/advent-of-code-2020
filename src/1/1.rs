use aoc2020::helper;

fn main() {
    let mut values = helper::get_numbers_from_file("1-input.txt");
    values.sort();

    'out_loop: for i_value in values.iter() {
        'inner_loop: for value in values.iter().rev() {
            if value + i_value == 2020 {
                helper::print_answer("1-1", value * i_value);
                break 'out_loop;
            }

            if value + i_value < 2020 {
                break 'inner_loop;
            }
        }
    }
}
