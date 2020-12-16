use aoc2020::helper;

mod ticketprocessor;
use ticketprocessor::TicketProcessor;

fn main() {
    let mut ticket_processor = TicketProcessor::new();
    ticket_processor.load("16-input.txt");
    ticket_processor.remove_invalid_tickets();
    println!("{}", ticket_processor.multiply_departures());
}
