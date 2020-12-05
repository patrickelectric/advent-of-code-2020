use aoc2020::helper;

mod ticket;

fn main() {
    let content = helper::get_input_file("5-input.txt");
    let mut ids = vec![];
    //TODO: Move to map, filter |p, n| for p != n-1
    for line in content.lines() {
        let (row_number, column_number) = ticket::get_seat_from_ticket(line);
        let id = row_number * 8 + column_number;
        ids.push(id);
    }

    ids.sort();
    let mut previous_id = ids[0] - 1;
    for id in ids.iter() {
        if *id != previous_id + 1 {
            helper::print_answer("5-2", id - 1);
            break;
        }
        previous_id = *id;
    }
}
