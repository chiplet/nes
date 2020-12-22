mod cpu;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::init();

    loop {
        cpu.tick().unwrap();
    }

    println!("ram[${:04x}] = ${:02x}", 0x200, cpu.ram[0x0200]);
}
