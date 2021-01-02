mod cpu;
mod bus;
mod nes;
mod util;
use crate::{
    nes::Nes
};

fn main() {
    let mut emulator = Nes::init();
    loop {
        emulator.tick().unwrap();
    }
//    let mut cpu = Cpu::init();
//
//    cpu.load_ines("./hexdumps/tests/nestest.nes").unwrap();
//    cpu.pc = 0xc000;
//    loop {
//        cpu.tick().unwrap();
//    }
}
