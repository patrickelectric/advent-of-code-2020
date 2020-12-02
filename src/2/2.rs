use aoc2020::helper;

mod password;

fn main() {
    let passwords = password::get_password_from_database("2-input.txt");

    let mut total = 0;
    for p in passwords {
        let mut chars = p.password.chars();
        let first_letter = chars.nth(p.first_number - 1);
        let second_letter = chars.nth(p.second_number - p.first_number - 1);

        if first_letter.is_none() || second_letter.is_none() {
            continue;
        }

        if first_letter.unwrap() == p.letter && second_letter.unwrap() != p.letter
            || first_letter.unwrap() != p.letter && second_letter.unwrap() == p.letter
        {
            total += 1;
        }
    }

    helper::print_answer("2-2", total);
}
