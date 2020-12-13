use aoc2020::helper;

mod scan;

fn main() {
    let passports = scan::scan_batch_file("4-input.txt");
    let total = passports.iter().filter(|&p| p.is_valid()).count();

    helper::print_answer("4-1", total as u64);
}
