use aoc2020::helper;

mod cpu;

fn main() {
    let mut cpu = cpu::CPU::new();
    let filename = "8-input.txt";
    cpu.load_program(filename);

    let instructions = cpu.instructions;

    'outside: for (address, instruction) in instructions.iter().enumerate() {
        let command = match instruction.command.as_ref() {
            "jmp" => "nop",
            "nop" => "jmp",
            _ => continue 'outside,
        };

        let patch = cpu::Patch {
            address: address,
            instruction: cpu::Instruction {
                command: command.into(),
                value: instruction.value,
            },
        };

        cpu.instructions = instructions.clone();
        cpu.apply_patch(patch);
        cpu.restart();
        let mut pc_values = vec![];

        while cpu.run_instruction() {
            if pc_values.contains(&cpu.pc) {
                continue 'outside;
            }

            pc_values.push(cpu.pc);
        }
        println!("Done!");
        break;
    }

    helper::print_answer("8-2", cpu.accumulator as u64);
}
