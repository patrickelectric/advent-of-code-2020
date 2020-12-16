use aoc2020::helper;

mod ticketprocessor;
use ticketprocessor::TicketProcessor;

fn main() {
    let mut ticket_processor = TicketProcessor::new();
    ticket_processor.load("16-input.txt");
    helper::print_answer("16-1", ticket_processor.error_rate());
}
