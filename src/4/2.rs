use aoc2020::helper;

mod scan;

fn main() {
    let passports = scan::scan_batch_file("4-input.txt");
    let total = passports.iter().filter(|&p| p.has_valid_data()).count();

    helper::print_answer("4-2", total as u64);
}
