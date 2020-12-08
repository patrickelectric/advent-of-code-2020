use aoc2020::helper;

use regex::Regex;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub command: String,
    pub value: i64,
}

pub struct Patch {
    pub address: usize,
    pub instruction: Instruction,
}

pub struct CPU {
    pub pc: usize, // program counter
    pub instructions: Vec<Instruction>,
    // registers
    pub accumulator: i64,
}

impl CPU {
    pub fn new() -> Self {
        return Self {
            pc: 0,
            instructions: vec![],
            accumulator: 0,
        };
    }

    fn compile_instruction(asm: &str) -> Instruction {
        let re = Regex::new(r"(?P<command>\S{3}) (?P<value>[+|-]\d+)").unwrap();

        let captured = re.captures(&asm);
        match captured {
            Some(content) => {
                return Instruction {
                    command: content.name("command").unwrap().as_str().into(),
                    value: content.name("value").unwrap().as_str().parse().unwrap(),
                }
            }
            _ => unreachable!("Failed to process asm line: {}", asm),
        };
    }

    pub fn apply_patch(&mut self, patch: Patch) {
        self.instructions[patch.address] = patch.instruction;
    }

    pub fn restart(&mut self) {
        self.pc = 0;
        self.accumulator = 0;
    }

    pub fn load_program(&mut self, filename: &str) {
        let program = helper::get_input_file(filename);
        self.instructions = program
            .lines()
            .map(|line| CPU::compile_instruction(line))
            .collect();
    }

    pub fn run_instruction(&mut self) -> bool {
        if self.pc >= self.instructions.len() {
            return false;
        }

        let instruction = &self.instructions[self.pc];
        match instruction.command.as_ref() {
            "acc" => {
                self.accumulator += instruction.value;
                self.pc += 1;
            }
            "jmp" => self.pc = (instruction.value + self.pc as i64) as usize,
            "nop" => self.pc += 1,
            _ => {}
        }

        return true;
    }
}
