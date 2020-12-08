use aoc2020::helper;

mod cpu;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.load_program("8-input.txt");
    let mut pc_values = vec![];

    while cpu.run_instruction() {
        if pc_values.contains(&cpu.pc) {
            break;
        }

        pc_values.push(cpu.pc);
    }

    helper::print_answer("8-1", cpu.accumulator as u64);
}
