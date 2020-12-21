mod isa;
use crate::cpu::isa::{Instruction, AddrMode};
use std::fmt;

// 7  bit  0
// ---- ----
// NVss DIZC
// |||| ||||
// |||| |||+- Carry
// |||| ||+-- Zero
// |||| |+--- Interrupt Disable
// |||| +---- Decimal
// ||++------ No CPU effect, see: the B flag
// |+-------- Overflow
// +--------- Negative

#[derive(Debug)]
pub struct StatusRegister {
    carry:              bool,
    zero:               bool,
    interrupt_disable:  bool,
    // decimal
    // reserved
    // B-flag (not in use)
    overflow:           bool,
    negative:           bool,
}
impl StatusRegister {
    fn init() -> Self {
        StatusRegister {
            carry: false,
            zero: false,
            interrupt_disable: true,
            overflow: false,
            negative: false,
        }
    }

    // set and clear Status Register bits
    pub fn set_carry(&mut self) { self.carry = true; }
    pub fn clear_carry(&mut self) { self.carry = false; }
    pub fn set_zero(&mut self) { self.zero = true; }
    pub fn clear_zero(&mut self) { self.zero = false; }
    pub fn set_interrupt_disable(&mut self) { self.interrupt_disable = true; }
    pub fn clear_interrupt_disable(&mut self) { self.interrupt_disable = false; }
    pub fn set_overflow(&mut self) { self.overflow = true; }
    pub fn clear_overflow(&mut self) { self.overflow = false; }
    pub fn set_negative(&mut self) { self.negative = true; }
    pub fn clear_negative(&mut self) { self.negative = false; }

}

/*** CPU structure ***/
#[derive(Debug)]
pub struct CPU {
    // addressable memory space
    pub ram: [u8; 65536],

    /*** Register definitions ***/
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,  // TODO: should this be a struct?
    pub pc: u16,
    pub sr: StatusRegister,
}

impl CPU {
    pub fn init() -> Self {
        let sample_program = [0xa9, 0x00, 0x69, 0x03, 0x8d, 0x00, 0x02];
        let mut sample_ram = [0; 65536];
        for byte in sample_program.iter().enumerate() {
            sample_ram[byte.0] = *byte.1;
        }

        CPU {
            // zero out CPU memory
            // ram: [0; 65536],
            ram: sample_ram, // load sample program to $0000

            // init CPU registers
            acc: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            sr: StatusRegister::init(),
        }
    }

    // forward emulation by one clock cycle
    pub fn tick(&mut self) -> Result<(), String> {
        // Fetch
        // FIXME: hardcoded to 3 byte slices
        let next_index = self.pc as usize;
        let instruction_bytes = &self.ram[next_index..next_index+3];

        // Decode
        let instruction = Instruction::decode(instruction_bytes)?;

        // print current instruction and CPU state before execution
        println!("${:04x}: {:?} {}", self.pc, instruction, self);

        // Execute
        self.execute(&instruction);
        Ok(())
    }

    // execute a machine instruction
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {

            // Load Accumulator with Memory
            Instruction::LDA(data) => {
                match data.addr_mode {
                    AddrMode::Imm(value) => {
                        self.acc = value;
                    }
                    AddrMode::Zpg(addr) => {
                        self.acc = self.ram[addr as usize];
                    }
                    AddrMode::ZpgX(addr) => {
                        self.acc = self.ram[(addr + self.x) as usize];
                    }
                    AddrMode::Abs(addr) => {
                        self.acc = self.ram[addr as usize]
                    }
                    AddrMode::AbsX(addr) => {
                        self.acc = self.ram[(addr + self.x as u16) as usize];
                    }
                    AddrMode::AbsY(addr) => {
                        self.acc = self.ram[(addr + self.y as u16) as usize];
                    }
                    AddrMode::XInd(addr) => {
                        let indirect = self.ram[(addr + self.x) as usize] as usize;
                        self.acc = self.ram[indirect];
                    }
                    AddrMode::IndY(addr) => {
                        let indirect = self.ram[addr as usize] as usize;
                        self.acc = self.ram[indirect + self.y as usize];
                    }
                    _ => panic!("Illegal addressing mode for LDA!")
                }

                self.pc += data.size();
            }

            // Add Memory to Accumulator with Carry
            Instruction::ADC(data) => {
                match data.addr_mode {
                    AddrMode::Imm(value) => {
                        self.acc += value;
                    }
                    AddrMode::Zpg(addr) => {
                        self.acc += self.ram[addr as usize];
                    }
                    AddrMode::ZpgX(addr) => {
                        self.acc += self.ram[(addr + self.x) as usize];
                    }
                    AddrMode::Abs(addr) => {
                        self.acc += self.ram[addr as usize]
                    }
                    AddrMode::AbsX(addr) => {
                        self.acc += self.ram[(addr + self.x as u16) as usize];
                    }
                    AddrMode::AbsY(addr) => {
                        self.acc += self.ram[(addr + self.y as u16) as usize];
                    }
                    AddrMode::XInd(addr) => {
                        let indirect = self.ram[(addr + self.x) as usize] as usize;
                        self.acc += self.ram[indirect];
                    }
                    AddrMode::IndY(addr) => {
                        let indirect = self.ram[addr as usize] as usize;
                        self.acc += self.ram[indirect + self.y as usize];
                    }
                    _ => panic!("Illegal addressing mode for ADC!")
                }
                self.pc += data.size();
            }

            Instruction::STA(data) => {
                match data.addr_mode {
                    AddrMode::Zpg(addr) => {
                        self.ram[addr as usize] = self.acc;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[(addr + self.x) as usize] = self.acc;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[addr as usize] = self.acc;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[(addr + self.x as u16) as usize] = self.acc;
                    }
                    AddrMode::AbsY(addr) => {
                        self.ram[(addr + self.y as u16) as usize] = self.acc;
                    }
                    AddrMode::XInd(addr) => {
                        let indirect = self.ram[(addr + self.x) as usize] as usize;
                        self.ram[indirect] = self.acc;
                    }
                    AddrMode::IndY(addr) => {
                        let indirect = self.ram[addr as usize] as usize;
                        self.ram[indirect + self.y as usize] = self.acc;
                    }
                    _ => panic!("Illegal addressing mode for LDA!")
                }
                self.pc += data.size();
            }

            _ => panic!("Emulation for the instruction not yet implemented!\n  {:?}", instruction)

        }
    }
}

// TODO: format status register nicely
impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ACC=${:02x} X=${:02x} Y=${:02x} SP=${:02x}",
            self.acc, self.x, self.y, self.sp
        )
    }
}

struct Program {
    instructions: Vec<Instruction>,
}
impl Program {
    fn to_bytes(&self) -> Vec<u8> {
        panic!("Program serialization not implemented.");
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::isa::{Instruction, AddrMode};

    #[test]
    fn decode_opcode_0x69_valid() {
        let code: [u8; 2] = [0x69, 0x03];
        if let Instruction::ADC(data) =  Instruction::decode(&code).unwrap() {
            assert_eq!(0x69, data.opcode);
            if let AddrMode::Imm(imm) = data.addr_mode {
                assert_eq!(0x03, imm);
            }
        }
    }

    #[test]
    #[should_panic]
    fn decode_opcode_0x69_invalid() {
        let code: [u8; 1] = [0x69];
        Instruction::decode(&code).unwrap();
    }

}