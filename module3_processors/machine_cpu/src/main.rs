mod cpu;
use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    let program = [
        0x21, 0x05,
        0x22, 0x03,
        0x41, 0x02,
        0x31, 0xC8,
        0xF0, 0x00,
    ];

    cpu.load_program(&program);
    cpu.run();

    println!("Result in memory[200] = {}", cpu.mem[200]);
}
