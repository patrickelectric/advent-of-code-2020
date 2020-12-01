use aoc2020::helper;

fn main() {
    let values = helper::get_numbers_from_file("1-input.txt");

    // slow way, but good enough
    'out_loop: for (i_index, i_value) in values.iter().enumerate() {
        for (j_index, j_value) in values.iter().enumerate() {
            if j_index == i_index {
                continue;
            }

            for (index, value) in values.iter().enumerate() {
                if index == i_index || index == j_index {
                    continue;
                }

                if value + i_value + j_value == 2020 {
                    helper::print_answer("1-2", value * i_value * j_value);
                    break 'out_loop;
                }
            }
        }
    }
}
