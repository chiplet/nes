use crate::bus::{Bus, RamDevice, CpuRamDevice};
use std::rc::Rc;
use std::cell::RefCell;
use crate::cpu::Cpu;

mod test;

/// Representation of a full NES system
pub struct Nes {
    // system bus struct which also contains all of the connected devices (CPU, PPU, memories)
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,
}
impl Nes {
    /// Initialize a new system emulator instance and all subcomponents
    pub fn init() -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let mut cpu = Cpu::init(&bus);


        // init devices connected to system bus
        let mut cpu_ram = CpuRamDevice::new(&bus);    // 2KB internal CPU RAM
        let mut remaining_addr_space = RamDevice::new(&bus, 0x2000, 0xe000);

        // add devices to bus
        bus.borrow_mut().add(cpu_ram).expect("Could not add CPU RAM to system bus");
        bus.borrow_mut().add(remaining_addr_space).expect("Could not add remaining memory to system bus");

        cpu.load_ines("./hexdumps/tests/nestest.nes").unwrap();
        cpu.pc = 0xc000;

        Nes {
            bus,
            cpu
        }
    }

    /// Advance system emulation by one time step
    pub fn tick(&mut self) -> Result<(), String> {
        self.cpu.tick()
    }
}