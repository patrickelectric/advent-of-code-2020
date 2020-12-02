use aoc2020::helper;

mod password;

fn main() {
    let passwords = password::get_password_from_database("2-input.txt");

    let mut total = 0;
    for p in passwords {
        let letter_count = p.password.matches(p.letter).count();
        if p.first_number <= letter_count && letter_count <= p.second_number {
            total += 1;
        }
    }

    helper::print_answer("2-1", total);
}
