mod isa;
use crate::cpu::isa::{Instruction, AddrMode, InstructionType};
use std::fmt;
use std::num::Wrapping;
use crate::main;

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
        let sample_program: [u8; 8] = [0xa9, 0xff, 0x29, 0xaa, 0x29, 0x55, 0x90, 0xf8];
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

        // Execute
        println!("${:04x}: {}{}  // {}", self.pc, instruction, self, instruction.name.description);
        self.execute(&instruction);
        Ok(())
    }

    // execute single machine instruction
    fn execute(&mut self, instruction: &Instruction) {
        match instruction.ins_type {

            // Load Accumulator with Memory
            InstructionType::LDA => {
                self.a = self.get_operand(instruction);
                self.set_sr_nz(self.a);
            }

            // Set Carry Flag
            InstructionType::SEC => { self.sr.set_bit(CARRY_BIT); }

            // Clear Carry Flag
            InstructionType::CLC => { self.sr.clear_bit(CARRY_BIT); }

            // Increment Index X by One
            InstructionType::INX => {
                self.x = (Wrapping(self.x) + Wrapping(1)).0;
                self.set_sr_nz(self.x);
            }

            // Increment Index Y by One
            InstructionType::INY => {
                self.y = (Wrapping(self.y) + Wrapping(1)).0;
                self.set_sr_nz(self.y);
            }

            // Jump to New Location
            InstructionType::JMP => {
                let jump_addr = match &instruction.addr_mode {
                    AddrMode::Abs(addr) => *addr,
                    AddrMode::Ind(addr) => { panic!("Indirect jump addressing not implemented!") }
                    _ => panic!("Illegal addressing mode for JMP!")
                };
                self.pc = jump_addr;
                self.pc -= instruction.machine_code.len() as u16; // compensate for normal pc adjustment
            }

            // Add Memory to Accumulator with Carry
            InstructionType::ADC => {
                let operand = self.get_operand(instruction);

                let carry_in = self.sr.get_bit(CARRY_BIT);
                let carry_out = operand.get_bit(7) & self.a.get_bit(7);
                let carry_6 = operand.get_bit(6) & self.a.get_bit(6);

                let sum_wrapped = Wrapping(self.a) + Wrapping(operand) + Wrapping(carry_in);
                self.a = sum_wrapped.0;

                self.sr.assign_bit(OVERFLOW_BIT, carry_6 ^ carry_out);
                self.sr.assign_bit(CARRY_BIT, carry_out);
                self.set_sr_nz(self.a);
            }

            // AND Memory with Accumulator
            InstructionType::AND => {
                let operand = self.get_operand(instruction);

                self.a &= operand;
                self.set_sr_nz(self.a);
            }

            // ASL  Shift Left One Bit (Memory or Accumulator)
            // TODO: implement

            // Branch on Carry Clear
            InstructionType::BCC => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(CARRY_BIT) == 0 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            InstructionType::STA => {
                match &instruction.addr_mode {
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = self.a;
                    }
                    _ => panic!("Illegal addressing mode for STA!")
                }
            }

            _ => panic!("Emulation for the instruction not yet implemented!\n  {:?}", instruction)
        }

        // addition is wrapping since some branch instructions rely on this behavior
        self.pc = self.pc.wrapping_add(instruction.machine_code.len() as u16);
    }


    /*** common functionality used to implement instruction emulation ***/
    // get instruction operand according to the associated addressing mode
    // operand of relative addressing is also returned as u8
    fn get_operand(&self, instruction: &Instruction) -> u8 {
        match &instruction.addr_mode {
            AddrMode::A => {
                self.a
            }
            AddrMode::Abs(addr) => {
                self.ram[*addr as usize]
            }
            AddrMode::AbsX(addr) => {
                self.ram[(*addr + self.x as u16) as usize]
            }
            AddrMode::AbsY(addr) => {
                self.ram[(*addr + self.y as u16) as usize]
            }
            AddrMode::Imm(value) => {
                *value
            }
            AddrMode::Impl => {
                panic!("Calling get_operand() for implied addressing mode does not make sense.")
            }
            AddrMode::Ind(addr) => {
                let indirect = self.ram[*addr as usize] as usize;
                self.ram[indirect]
            }
            AddrMode::XInd(addr) => {
                let indirect = self.ram[(*addr + self.x) as usize] as usize;
                self.ram[indirect]
            }
            AddrMode::IndY(addr) => {
                let indirect = self.ram[*addr as usize] as usize;
                self.ram[indirect + self.y as usize]
            }
            AddrMode::Rel(value) => {
                *value as u8
            }
            AddrMode::Zpg(addr) => {
                self.ram[*addr as usize]
            }
            AddrMode::ZpgX(addr) => {
                self.ram[(*addr + self.x) as usize]
            }
            AddrMode::ZpgY(addr) => {
                self.ram[(*addr + self.y) as usize]
            }
        }
    }

    // set zero and negative flags based on value
    fn set_sr_nz(&mut self, value: u8) {
        self.sr.assign_bit(NEGATIVE_BIT, value.get_bit(7));
        match value {
            0 => self.sr.set_bit(ZERO_BIT),
            _ => self.sr.clear_bit(ZERO_BIT),
        }
    }
}
impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A:${:02x} X:${:02x} Y:${:02x} SP:${:02x} SR:{:08b}",
            self.a, self.x, self.y, self.sp, self.sr
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
}