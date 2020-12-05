pub fn get_seat_from_ticket(ticket: &str) -> (u64, u64) {
    let (rows, columns) = ticket.split_at(7);
    let row_number = rows.replace("F", "0").replace("B", "1");
    let column_number = columns.replace("L", "0").replace("R", "1");
    let row_number = u64::from_str_radix(&row_number, 2).unwrap();
    let column_number = u64::from_str_radix(&column_number, 2).unwrap();
    return (row_number, column_number);
}
