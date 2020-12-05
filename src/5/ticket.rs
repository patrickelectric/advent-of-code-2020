use aoc2020::helper;

pub fn get_id_from_ticket(ticket: &str) -> u64 {
    return u64::from_str_radix(
        &ticket
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"),
        2,
    )
    .unwrap();
}

pub fn get_tickets_id_from_file(filename: &str) -> Vec<u64> {
    let content = helper::get_input_file(filename);
    let mut tickets: Vec<u64> = content
        .lines()
        .map(|line| get_id_from_ticket(line))
        .collect();
    tickets.sort();
    return tickets;
}
