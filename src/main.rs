mod cpu;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::init();

    for i in 0..3 {
        // cpu.ram[0] = i;
        cpu.tick().unwrap();
        // cpu.pc = Wrapping(0);
    }

    println!("ram[${:04x}] = ${:02x}", 0x200, cpu.ram[0x0200]);
}
