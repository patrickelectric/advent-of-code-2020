use aoc2020::helper;

mod ticket;

fn main() {
    let ids = ticket::get_tickets_id_from_file("5-input.txt");
    let highest_id = ids.last().unwrap();
    helper::print_answer("5-1", *highest_id);
}
