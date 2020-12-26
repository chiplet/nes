mod cpu;
mod util;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::init();

    cpu.load_ines("./hexdumps/tests/nestest.nes").unwrap();
    cpu.pc = 0xc000;
    loop {
        cpu.tick();
    }

    // println!("ram[${:04x}] = ${:02x}", 0x200, cpu.ram[0x0200]);
}
