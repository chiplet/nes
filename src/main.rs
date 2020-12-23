mod cpu;
mod util;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::init();

    cpu.load_hexdump("./hexdumps/tmp.txt").unwrap();
    cpu.pc = 0x0600;

    loop {
        cpu.tick().unwrap();
    }

    // println!("ram[${:04x}] = ${:02x}", 0x200, cpu.ram[0x0200]);
}
