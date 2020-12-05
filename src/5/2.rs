use aoc2020::helper;

mod ticket;

fn main() {
    let ids = ticket::get_tickets_id_from_file("5-input.txt");
    let mut previous_id = ids[0];
    for id in ids.iter().skip(1) {
        if *id != previous_id + 1 {
            helper::print_answer("5-2", id - 1);
            break;
        }
        previous_id = *id;
    }
}
