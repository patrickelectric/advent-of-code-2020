use aoc2020::helper;

use std::time::Instant;

fn main() {
    let mut values = helper::get_numbers_from_file("1-input.txt");

    // slow way, but good enough
    let start = Instant::now();
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
                    helper::print_answer("slow way: 1-2", value * i_value * j_value);
                    println!("Took: {:?}", start.elapsed());
                    break 'out_loop;
                }
            }
        }
    }

    // Fast way
    let start = Instant::now();
    values.sort();
    let middle_position = values.iter().position(|value| *value > 1010).unwrap();
    let (minor_chunk, major_chunk) = values.split_at(middle_position);

    // 1. 3 lower than 1010
    for (index_1, minor_value_1) in minor_chunk.iter().enumerate() {
        for (index_2, minor_value_2) in minor_chunk.iter().enumerate().skip(index_1 + 1) {
            for minor_value_3 in minor_chunk.iter().skip(index_2 + 1) {
                if minor_value_1 + minor_value_2 + minor_value_3 == 2020 {
                    helper::print_answer(
                        "fast way: 1-2",
                        minor_value_1 * minor_value_2 * minor_value_3,
                    );
                    println!("Took: {:?}", start.elapsed());
                    std::process::exit(0);
                }
            }
        }
    }

    // 2. 2 lower than 1010 and 1 bigger than 1010
    'minor_loop_1: for (index, minor_value_1) in minor_chunk.iter().enumerate() {
        for minor_value_2 in minor_chunk.iter().skip(index + 1) {
            if minor_value_1 + minor_value_2 > 1010 {
                continue 'minor_loop_1;
            }

            for major_value in major_chunk.iter() {
                if minor_value_1 + minor_value_2 + major_value > 2020 {
                    break;
                }
                if minor_value_1 + minor_value_2 + major_value == 2020 {
                    helper::print_answer(
                        "fast way: 1-2",
                        minor_value_1 * minor_value_2 * major_value,
                    );
                    println!("Took: {:?}", start.elapsed());
                    std::process::exit(0);
                }
            }
        }
    }
}
