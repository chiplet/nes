mod cpu;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::init();

    println!("{:#?}", cpu.sr);

    for i in 0..3 {
        cpu.tick();
    }

    println!("{}", cpu.ram[0x0200]);
}
