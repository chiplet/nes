mod isa;
use crate::cpu::isa::{Instruction, AddrMode, InstructionType};
use std::fmt;

// Status Register bit descriptions
//
//   7  bit  0
//   ---- ----
//   NVss DIZC
//   |||| ||||
//   |||| |||+- Carry
//   |||| ||+-- Zero
//   |||| |+--- Interrupt Disable
//   |||| +---- Decimal
//   ||++------ No CPU effect, see: the B flag
//   |+-------- Overflow
//   +--------- Negative
const CARRY_BIT: u8 = 0;
const ZERO_BIT: u8 = 1;
const INT_DISABLE_BIT: u8 = 2;
const DECIMAL_BIT: u8 = 3;
const OVERFLOW_BIT: u8 = 6;
const NEGATIVE_BIT: u8 = 7;


trait BitOps {
    // common bit operations
    fn set_bit(&mut self, index: u8);
    fn clear_bit(&mut self, index: u8);
    fn get_bit(&self, index: u8) -> u8;
    fn assign_bit(&mut self, index: u8, value: u8);
}
impl BitOps for u8 {
    fn set_bit(&mut self, index: u8) {
        if index < 0 || index > 7 {
            panic!("Invalid bit index");
        }
        *self |= 1 << index;
    }
    fn clear_bit(&mut self, index: u8) {
        if index < 0 || index > 7 {
            panic!("Invalid bit index");
        }
        *self &= !(1 << index);
    }
    fn get_bit(&self, index: u8) -> u8 {
        if index < 0 || index > 7 {
            panic!("Invalid bit index");
        }
        (*self >> index) & 1u8
    }
    fn assign_bit(&mut self, index: u8, value: u8) {
        if index < 0 || index > 7 {
            panic!("Invalid bit index");
        }
        if value < 0 || value > 1 {
            panic!("Bit can only be assigned values 0 or 1");
        }
        *self = *self & !(1u8 << index);    // clear bit at index
        *self |= value << index;            // assign value to bit at index
    }
}


/*** CPU structure ***/
#[derive(Debug)]
pub struct CPU {
    // addressable memory space
    pub ram: [u8; 65536],

    // registers
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub sr: u8,
}
impl CPU {
    pub fn init() -> Self {
        let sample_program = [0xa9, 0x00, 0x69, 0x03, 0x8d, 0x00, 0x02];
        let mut sample_ram = [0; 65536];
        for byte in sample_program.iter().enumerate() {
            sample_ram[byte.0] = *byte.1;
        }

        // enable interrupt_disable bit on startup
        let mut init_sr = 0;
        init_sr.set_bit(INT_DISABLE_BIT);

        CPU {
            // zero out CPU memory
            // ram: [0; 65536],
            ram: sample_ram, // load sample program to $0000

            // init CPU registers
            a: 0,
            x: 0,
            y: 0,
            sp: 0u8,
            pc: 0u16,
            sr: init_sr,
        }
    }

    // forward emulation by one clock cycle
    pub fn tick(&mut self) -> Result<(), String> {
        // Fetch
        let next_index = self.pc as usize;
        let instruction_bytes = &self.ram[next_index..next_index+3];

        // Decode
        let instruction = Instruction::from(instruction_bytes)?;

        // print current instruction and CPU state before execution
        println!("${:04x}: {}  {}      {}", self.pc, instruction, self, instruction.name.description);

        // Execute
        self.execute(&instruction);
        Ok(())
    }

    // execute single machine instruction
    fn execute(&mut self, instruction: &Instruction) {
        match instruction.ins_type {

            // Load Accumulator with Memory
            InstructionType::LDA => {
                match &instruction.addr_mode {
                    AddrMode::Imm(value) => {
                        self.a = *value;
                    }
                    /*AddrMode::Zpg(addr) => {
                        self.a = self.ram[*addr as usize];
                    }
                    AddrMode::ZpgX(addr) => {
                        self.a = self.ram[(*addr + self.x) as usize];
                    }
                    AddrMode::Abs(addr) => {
                        self.a = self.ram[*addr as usize];
                    }
                    AddrMode::AbsX(addr) => {
                        self.a = self.ram[(*addr + self.x as u16) as usize];
                    }
                    AddrMode::AbsY(addr) => {
                        self.a = self.ram[(addr + self.y as u16) as usize];
                    }
                    AddrMode::XInd(addr) => {
                        let indirect = self.ram[(addr + self.x) as usize] as usize;
                        self.a = self.ram[indirect];
                    }
                    AddrMode::IndY(addr) => {
                        let indirect = self.ram[*addr as usize] as usize;
                        self.a = self.ram[indirect + self.y as usize];
                    }*/
                    _ => panic!("Illegal addressing mode for LDA!")
                }
            }

            // Add Memory to Accumulator with Carry
            InstructionType::ADC => {
                match &instruction.addr_mode {
                    AddrMode::Imm(value) => {
                        // self.a += Register::from(*value);
                        ()
                    }
/*                    AddrMode::Zpg(addr) => {
                        self.a += self.ram[*addr as usize];
                    }
                    AddrMode::ZpgX(addr) => {
                        self.a += self.ram[(*addr + self.x) as usize];
                    }
                    AddrMode::Abs(addr) => {
                        self.a += self.ram[*addr as usize];
                    }
                    AddrMode::AbsX(addr) => {
                        self.a += self.ram[(*addr + self.x as u16) as usize];
                    }
                    AddrMode::AbsY(addr) => {
                        self.a += self.ram[(*addr + self.y as u16) as usize];
                    }
                    AddrMode::XInd(addr) => {
                        let indirect = self.ram[(*addr + self.x) as usize] as usize;
                        self.a += self.ram[indirect];
                    }
                    AddrMode::IndY(addr) => {
                        let indirect = self.ram[*addr as usize] as usize;
                        self.a += self.ram[indirect + self.y as usize];
                    }*/
                    _ => panic!("Illegal addressing mode for ADC!")
                }
            }

            InstructionType::STA => {
                match &instruction.addr_mode {
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = self.a;
                    }
                    _ => panic!("Illegal addressing mode for LDA!")
                }
            }

            _ => panic!("Emulation for the instruction not yet implemented!\n  {:?}", instruction)
        }
        self.pc += instruction.machine_code.len() as u16;
    }

    /** status register bit manipulation **/
    fn set_sr_carry(&mut self) { self.sr.set_bit(CARRY_BIT)}
    fn set_sr_interrupt_disable(&mut self) { self.sr.set_bit(INT_DISABLE_BIT)}
    fn set_sr_negative(&mut self) { self.sr.set_bit(NEGATIVE_BIT)}
    fn set_sr_overflow(&mut self) { self.sr.set_bit(OVERFLOW_BIT)}
    fn set_sr_zero(&mut self) { self.sr.set_bit(ZERO_BIT)}
    fn set_sr_decimal(&mut self) { self.sr.set_bit(DECIMAL_BIT)}

    fn clear_sr_carry(&mut self) { self.sr.clear_bit(CARRY_BIT)}
    fn clear_sr_interrupt_disable(&mut self) { self.sr.clear_bit(INT_DISABLE_BIT)}
    fn clear_sr_negative(&mut self) { self.sr.clear_bit(NEGATIVE_BIT)}
    fn clear_sr_overflow(&mut self) { self.sr.clear_bit(OVERFLOW_BIT)}
    fn clear_sr_zero(&mut self) { self.sr.clear_bit(ZERO_BIT)}
    fn clear_sr_decimal(&mut self) { self.sr.clear_bit(DECIMAL_BIT)}

    fn get_sr_carry(&mut self) -> u8 { self.sr.get_bit(CARRY_BIT)}
    fn get_sr_interrupt_disable(&mut self) -> u8 { self.sr.get_bit(INT_DISABLE_BIT)}
    fn get_sr_negative(&mut self) -> u8 { self.sr.get_bit(NEGATIVE_BIT)}
    fn get_sr_overflow(&mut self) -> u8 { self.sr.get_bit(OVERFLOW_BIT)}
    fn get_sr_zero(&mut self) -> u8 { self.sr.get_bit(ZERO_BIT)}
    fn get_sr_decimal(&mut self) -> u8 { self.sr.get_bit(DECIMAL_BIT)}

    fn assign_sr_carry(&mut self) -> u8 { self.sr.get_bit(CARRY_BIT)}
    fn assign_sr_interrupt_disable(&mut self) -> u8 { self.sr.get_bit(INT_DISABLE_BIT)}
    fn assign_sr_negative(&mut self) -> u8 { self.sr.get_bit(NEGATIVE_BIT)}
    fn assign_sr_overflow(&mut self) -> u8 { self.sr.get_bit(OVERFLOW_BIT)}
    fn assign_sr_zero(&mut self) -> u8 { self.sr.get_bit(ZERO_BIT)}
    fn assign_sr_decimal(&mut self) -> u8 { self.sr.get_bit(DECIMAL_BIT)}


    // common functionality used to implement instruction emulation
    fn set_sr_nz(&mut self, value: u8) {
        self.sr.assign_bit(NEGATIVE_BIT, value.get_bit(7));
        match value {
            0 => self.sr.set_bit(ZERO_BIT),
            _ => self.sr.clear_bit(ZERO_BIT),
        }
    }
}
impl fmt::Display for CPU {
    // TODO: format status register nicely
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A:${:02x} X:${:02x} Y:${:02x} SP:${:02x}",
            self.a, self.x, self.y, self.sp
        )
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{BitOps, CPU};

    #[test]
    fn get_bit() {
        let r = 0x55;
        assert_eq!(1, r.get_bit(0));
        assert_eq!(0, r.get_bit(1));
        assert_eq!(1, r.get_bit(2));
        assert_eq!(0, r.get_bit(3));
        assert_eq!(1, r.get_bit(4));
        assert_eq!(0, r.get_bit(5));
        assert_eq!(1, r.get_bit(6));
        assert_eq!(0, r.get_bit(7));
    }

    #[test]
    fn set_bit() {
        let mut r = 0x00;
        r.set_bit(0);
        r.set_bit(2);
        r.set_bit(4);
        r.set_bit(6);

        assert_eq!(0x55, r);
    }

    #[test]
    fn clear_bit() {
        let mut r = 0xff;
        r.clear_bit(0);
        r.clear_bit(2);
        r.clear_bit(4);
        r.clear_bit(6);

        assert_eq!(0xaa, r);
    }

    #[test]
    fn assign_bit() {
        let mut r = 0;
        r.assign_bit(0, 1);
        assert_eq!(r, 1);
        r.assign_bit(0, 0);
        assert_eq!(r, 0);

        let mut r = 0xaa;               // 1010_1010
        r.assign_bit(7, 0);     // 0010_1010
        assert_eq!(r, 0x2a);
        r.assign_bit(6, 1);     // 0110_1010
        assert_eq!(r, 0x6a);
        r.assign_bit(4, 1);     // 0111_1010
        assert_eq!(r, 0x7a);
    }

    #[test]
    fn sr_helpers() {
        let mut cpu = CPU::init();

        // test bits individually
        cpu.sr = 0x00;
        cpu.set_sr_carry();
        assert_eq!(1, cpu.get_sr_carry());
        cpu.sr = 0xff;
        cpu.clear_sr_carry();
        assert_eq!(0, cpu.get_sr_carry());

        cpu.sr = 0x00;
        cpu.set_sr_interrupt_disable();
        assert_eq!(1, cpu.get_sr_interrupt_disable());
        cpu.sr = 0xff;
        cpu.clear_sr_interrupt_disable();
        assert_eq!(0, cpu.get_sr_interrupt_disable());

        cpu.sr = 0x00;
        cpu.set_sr_negative();
        assert_eq!(1, cpu.get_sr_negative());
        cpu.sr = 0xff;
        cpu.clear_sr_negative();
        assert_eq!(0, cpu.get_sr_negative());

        cpu.sr = 0x00;
        cpu.set_sr_overflow();
        assert_eq!(1, cpu.get_sr_overflow());
        cpu.sr = 0xff;
        cpu.clear_sr_overflow();
        assert_eq!(0, cpu.get_sr_overflow());

        cpu.sr = 0x00;
        cpu.set_sr_zero();
        assert_eq!(1, cpu.get_sr_zero());
        cpu.sr = 0xff;
        cpu.clear_sr_zero();
        assert_eq!(0, cpu.get_sr_zero());

        cpu.sr = 0x00;
        cpu.set_sr_decimal();
        assert_eq!(1, cpu.get_sr_decimal());
        cpu.sr = 0xff;
        cpu.clear_sr_decimal();
        assert_eq!(0, cpu.get_sr_decimal());

        // set all bits
        cpu.sr = 0x00;
        cpu.set_sr_carry();
        cpu.set_sr_interrupt_disable();
        cpu.set_sr_negative();
        cpu.set_sr_overflow();
        cpu.set_sr_zero();
        cpu.set_sr_decimal();
        assert_eq!(cpu.sr, 0b1100_1111);
    }
}