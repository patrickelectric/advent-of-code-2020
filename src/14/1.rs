use aoc2020::helper;

use std::collections::HashMap;

use regex::Regex;

struct PortMemory {
    memory: HashMap<u64, u64>,
    mask_0: u64,
    mask_1: u64,
    regex_mask: Regex,
    regex_data: Regex,
}

impl PortMemory {
    pub fn new() -> Self {
        return Self {
            memory: HashMap::default(),
            mask_0: (1 << 36 - 1),
            mask_1: (1 << 36 - 1),
            regex_mask: Regex::new(r"mask = (?P<mask>\S{36})").unwrap(),
            regex_data: Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap(),
        };
    }

    pub fn set_mask(&mut self, mask: &str) {
        let bits = mask.chars().enumerate();
        let zeros: u64 = bits
            .clone()
            .filter(|(_, value)| value == &'0')
            .map(|(index, _)| 1 << (35 - index as u64))
            .sum();
        self.mask_0 = !zeros;
        self.mask_1 = bits
            .filter(|(_, value)| value == &'1')
            .map(|(index, _)| 1 << (35 - index as u64))
            .sum();
    }

    pub fn apply_mask(&self, value: u64) -> u64 {
        return (value | self.mask_1) & self.mask_0;
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
        self.memory.insert(address, self.apply_mask(value));
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
        //mask
        port_memory.set_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(port_memory.apply_mask(11), 73);
        assert_eq!(port_memory.apply_mask(101), 101);
        assert_eq!(port_memory.apply_mask(0), 64);

        //set
        port_memory.run_instruction("mem[8] = 11");
        port_memory.run_instruction("mem[7] = 101");
        port_memory.run_instruction("mem[8] = 0");
        assert_eq!(port_memory.sum(), 165);
    }
}
