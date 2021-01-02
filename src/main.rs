mod cpu;
mod bus;
mod util;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::init();

    cpu.load_ines("./hexdumps/tests/nestest.nes").unwrap();
    cpu.pc = 0xc000;
    loop {
        cpu.tick().unwrap();
    }
}
