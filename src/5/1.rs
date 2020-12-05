use aoc2020::helper;

mod ticket;

fn main() {
    let content = helper::get_input_file("5-input.txt");
    let mut highest_id = 0;
    for line in content.lines() {
        let (row_number, column_number) = ticket::get_seat_from_ticket(line);
        let id = row_number * 8 + column_number;
        if id > highest_id {
            highest_id = id;
        }
    }

    helper::print_answer("5-1", highest_id);
}
