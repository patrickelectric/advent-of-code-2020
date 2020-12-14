use aoc2020::helper;

use std::collections::HashMap;

use regex::Regex;

struct PortMemory {
    memory: HashMap<u64, u64>,
    mask: String,
    regex_mask: Regex,
    regex_data: Regex,
}

impl PortMemory {
    pub fn new() -> Self {
        return Self {
            memory: HashMap::default(),
            mask: String::default(),
            regex_mask: Regex::new(r"mask = (?P<mask>\S{36})").unwrap(),
            regex_data: Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap(),
        };
    }

    pub fn set_mask(&mut self, mask: &str) {
        self.mask = mask.to_string();
    }

    fn get_addresses(mask: &str) -> Vec<u64> {
        if !mask.contains('X') {
            return vec![u64::from_str_radix(mask, 2).unwrap()];
        }

        let mut first = PortMemory::get_addresses(&mask.replacen('X', &"0", 1));
        let mut second = PortMemory::get_addresses(&mask.replacen('X', &"1", 1));
        let mut addresses = vec![];
        addresses.append(&mut first);
        addresses.append(&mut second);
        return addresses;
    }

    pub fn apply_mask(&self, address: u64) -> Vec<u64> {
        let address = format!("{:036b}", address);

        let logic = |(address, mask)| -> char {
            return match mask {
                '0' => address,
                _ => mask,
            };
        };

        let zip = address.chars().zip(self.mask.chars());
        let masked_address: String = zip.map(logic).collect();
        let addresses = PortMemory::get_addresses(&masked_address);
        return addresses;
    }

    pub fn run_instruction(&mut self, instruction: &str) {
        if instruction.contains("mask") {
            let mask = self
                .regex_mask
                .captures(instruction)
                .unwrap()
                .name("mask")
                .unwrap()
                .as_str();
            self.set_mask(mask);
            return;
        }

        let capture = self.regex_data.captures(instruction).unwrap();
        let address = capture
            .name("address")
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let value = capture
            .name("value")
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();

        for address in self.apply_mask(address) {
            self.memory.insert(address, value);
        }
    }

    pub fn sum(&self) -> u64 {
        return self.memory.values().sum();
    }
}

fn main() {
    let instructions = helper::get_input_file("14-input.txt");

    let mut port_memory = PortMemory::new();
    for instruction in instructions.lines() {
        port_memory.run_instruction(instruction);
    }

    helper::print_answer("14-1", port_memory.sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut port_memory = PortMemory::new();
        port_memory.run_instruction("mask = 000000000000000000000000000000X1001X");
        port_memory.run_instruction("mem[42] = 100");
        port_memory.run_instruction("mask = 00000000000000000000000000000000X0XX");
        port_memory.run_instruction("mem[26] = 1");
        assert_eq!(port_memory.sum(), 208);
    }
}
