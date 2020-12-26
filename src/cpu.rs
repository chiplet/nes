mod isa;
use crate::cpu::isa::{Instruction, AddrMode, InstructionType};
use crate::util;
use std::{
    fs,
    fmt,
    num::Wrapping
};


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
        if index > 7 {
            panic!("Invalid bit index");
        }
        *self |= 1 << index;
    }
    fn clear_bit(&mut self, index: u8) {
        if index > 7 {
            panic!("Invalid bit index");
        }
        *self &= !(1 << index);
    }
    fn get_bit(&self, index: u8) -> u8 {
        if index > 7 {
            panic!("Invalid bit index");
        }
        (*self >> index) & 1u8
    }
    fn assign_bit(&mut self, index: u8, value: u8) {
        if index > 7 {
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
    pub ram: Vec<u8>,

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
        // enable interrupt_disable bit on startup
        let mut init_sr = 0;
        init_sr.set_bit(INT_DISABLE_BIT);

        CPU {
            // zero out CPU memory
            ram: vec![0; 65536],

            // init CPU registers
            a: 0,
            x: 0,
            y: 0,
            sp: 0xfd,
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
        println!("{:04X}  {}{}", self.pc, instruction, self);
        self.execute(&instruction);
        Ok(())
    }

    // read hexdump generated by easy6502 assembler and load bytes to memory
    pub fn load_hexdump(&mut self, filename: &str) -> Result<(), String> {
        let lines = match util::read_lines(filename) {
            Ok(lines) => Ok(lines),
            Err(e) => Err(format!("{}", e)),
        }?;

        println!("Loading memory from hexdump file: {}", filename);
        for line in lines {
            let line = line.unwrap();
            println!("  {}", line);

            // parse target address and bytes from line
            let values = line
                .trim()
                .split(" ")
                .collect::<Vec<&str>>();

            // extract starting address and program bytes
            let addr = u16::from_str_radix(&values[0][0..4], 16).unwrap();
            let bytes = &values[1..]
                .iter()
                .map(|x| u8::from_str_radix(x, 16).unwrap())
                .collect::<Vec<u8>>();

            // copy bytes to memory
            for b in bytes.iter().enumerate() {
                self.ram[addr as usize + b.0] = *b.1;
            }
        }
        println!();

        Ok(())
    }

    // read raw bytes from a binary file and load bytes to memory
    // start writing to ram from offset
    pub fn load_ines(&mut self, filename: &str) -> Result<(), String> {
        // FIXME: currently hardcoded to load nestest.nes
        // println!("Loading memory from ines file: {}", filename);
        let bytes = match fs::read(filename) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(format!("{}", e)),
        }?;
        // println!();

        // TODO: add error handling

        for i in 0..0x4000 {
            self.ram[0xc000 + i] = bytes[i + 0x10];
        }

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

            // Load Index X with Memory
            InstructionType::LDX => {
                self.x = self.get_operand(instruction);
                self.set_sr_nz(self.x);
            }

            // Load Index Y with Memory
            InstructionType::LDY => {
                self.y = self.get_operand(instruction);
                self.set_sr_nz(self.y);
            }

            // Shift One Bit Right (Memory or Accumulator)
            InstructionType::LSR => {
                let operand = self.get_operand(instruction);
                let result = operand >> 1;

                // rightmost bit gets assigned to carry
                self.sr.assign_bit(CARRY_BIT, operand.get_bit(0));
                self.set_sr_nz(result);

                match &instruction.addr_mode {
                    AddrMode::A => {
                        self.a = result;
                    }
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    _ => panic!("Illegal addressing mode for LSR!")
                }
            }

            // No Operation
            InstructionType::NOP => {}

            // OR Memory with Accumulator
            InstructionType::ORA => {
                let operand = self.get_operand(instruction);
                self.a |= operand;
                self.set_sr_nz(self.a);
            }

            // Push Accumulator on Stack
            InstructionType::PHA => {
                self.stack_push_byte(self.a);
            }

            // Push Processor Status on Stack
            InstructionType::PHP => {
                self.stack_push_byte(self.sr);
            }

            // Pull Accumulator from Stack
            InstructionType::PLA => {
                self.a = self.stack_pop_byte();
                self.set_sr_nz(self.a);
            }

            // Pull Processor Status from Stack
            InstructionType::PLP => {
                self.sr = self.stack_pop_byte();
            }

            // Rotate One Bit Left (Memory or Accumulator)
            InstructionType::ROL => {
                let operand = self.get_operand(instruction);
                let mut result = operand << 1;
                result.assign_bit(0, self.sr.get_bit(CARRY_BIT));

                // rightmost bit gets assigned to carry
                self.sr.assign_bit(CARRY_BIT, operand.get_bit(7));
                self.set_sr_nz(result);

                match &instruction.addr_mode {
                    AddrMode::A => {
                        self.a = result;
                    }
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    _ => panic!("Illegal addressing mode for ROL!")
                }
            }

            // Rotate One Bit Right (Memory or Accumulator)
            InstructionType::ROR => {
                let operand = self.get_operand(instruction);
                let mut result = operand >> 1;
                result.assign_bit(7, self.sr.get_bit(CARRY_BIT));

                // rightmost bit gets assigned to carry
                self.sr.assign_bit(CARRY_BIT, operand.get_bit(0));
                self.set_sr_nz(result);

                match &instruction.addr_mode {
                    AddrMode::A => {
                        self.a = result;
                    }
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    _ => panic!("Illegal addressing mode for ROR!")
                }
            }

            // Return from Interrupt
            InstructionType::RTI => {
                self.sr = self.stack_pop_byte();
                self.pc = self.stack_pop();
                self.pc -= instruction.machine_code.len() as u16; // compensate for normal pc adjustment
            }

            // Return from Subroutine
            InstructionType::RTS => {
                self.pc = self.stack_pop()+1;
                self.pc -= instruction.machine_code.len() as u16; // compensate for normal pc adjustment
            }

            // Subtract Memory from Accumulator with Borrow
            InstructionType::SBC => {
                let operand = !self.get_operand(instruction);
                let carry_in = self.sr.get_bit(CARRY_BIT);

                // set overflow flag if appropriate
                let carry_in_added_i8 = (self.a as i8).overflowing_add(carry_in as i8);
                let operand_added_i8 = carry_in_added_i8.0.overflowing_add(operand as i8);
                let overflow: u8 = match carry_in_added_i8.1 | operand_added_i8.1 {
                    false => 0u8,
                    true => 1u8,
                };

                // compute sum and carry out flag
                let carry_in_added = self.a.overflowing_add(carry_in);
                let operand_added = carry_in_added.0.overflowing_add(operand);
                let carry_out: u8 = match carry_in_added.1 | operand_added.1 {
                    false => 0,
                    true => 1,
                };

                self.a = operand_added.0;
                self.sr.assign_bit(OVERFLOW_BIT, overflow);
                self.sr.assign_bit(CARRY_BIT, carry_out);
                self.set_sr_nz(self.a);
            }

            // Set Carry Flag
            InstructionType::SEC => { self.sr.set_bit(CARRY_BIT); }

            // Set Decimal Flag
            InstructionType::SED => { self.sr.set_bit(DECIMAL_BIT); }

            // Set Interrupt Disable Status
            InstructionType::SEI => { self.sr.set_bit(INT_DISABLE_BIT); }

            // Add Memory to Accumulator with Carry
            InstructionType::ADC => {
                let operand = self.get_operand(instruction);
                let carry_in = self.sr.get_bit(CARRY_BIT);

                // set overflow flag if appropriate
                let carry_in_added_i8 = (self.a as i8).overflowing_add(carry_in as i8);
                let operand_added_i8 = carry_in_added_i8.0.overflowing_add(operand as i8);
                let overflow: u8 = match carry_in_added_i8.1 | operand_added_i8.1 {
                    false => 0u8,
                    true => 1u8,
                };

                // compute sum and carry out flag
                let carry_in_added = self.a.overflowing_add(carry_in);
                let operand_added = carry_in_added.0.overflowing_add(operand);
                let carry_out: u8 = match carry_in_added.1 | operand_added.1 {
                    false => 0,
                    true => 1,
                };

                self.a = operand_added.0;
                self.sr.assign_bit(OVERFLOW_BIT, overflow);
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
            InstructionType::ASL => {
                let operand = self.get_operand(instruction);
                let result = operand << 1;

                // rightmost bit gets assigned to carry
                self.sr.assign_bit(CARRY_BIT, operand.get_bit(7));
                self.set_sr_nz(result);

                match &instruction.addr_mode {
                    AddrMode::A => {
                        self.a = result;
                    }
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    _ => panic!("Illegal addressing mode for ASL!")
                }
            }

            // Branch on Carry Clear
            InstructionType::BCC => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(CARRY_BIT) == 0 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // Branch on Carry Clear
            InstructionType::BCS => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(CARRY_BIT) == 1 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // Branch on Result Zero
            InstructionType::BEQ => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(ZERO_BIT) == 1 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // BIT  Test Bits in Memory with Accumulator
            InstructionType::BIT => {
                let operand = self.get_operand(instruction);
                self.sr.assign_bit(NEGATIVE_BIT, operand.get_bit(NEGATIVE_BIT));
                self.sr.assign_bit(OVERFLOW_BIT, operand.get_bit(OVERFLOW_BIT));
                match self.a & operand {
                    0 => self.sr.set_bit(ZERO_BIT),
                    _ => self.sr.clear_bit(ZERO_BIT),
                }
            }

            // Branch on Result Minus
            InstructionType::BMI => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(NEGATIVE_BIT) == 1 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // Branch on Result not Zero
            InstructionType::BNE => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(ZERO_BIT) == 0 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // Branch on Result Plus
            InstructionType::BPL => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(NEGATIVE_BIT) == 0 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // Force Break
            InstructionType::BRK => {
                panic!("TODO: implement CPU interrupts");
                self.stack_push(self.pc+2);
                self.stack_push_byte(self.sr);
                self.sr.set_bit(INT_DISABLE_BIT);
            }

            // Branch on Overflow Clear
            InstructionType::BVC => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(OVERFLOW_BIT) == 0 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // Branch on Overflow Set
            InstructionType::BVS => {
                let operand = self.get_operand(instruction);
                if self.sr.get_bit(OVERFLOW_BIT) == 1 {
                    self.pc = self.pc.wrapping_add((operand as i8) as u16);
                }
            }

            // Clear Carry Flag
            InstructionType::CLC => { self.sr.clear_bit(CARRY_BIT); }

            // Clear Decimal Mode
            InstructionType::CLD => { self.sr.clear_bit(DECIMAL_BIT); }

            // Clear Interrupt Disable Bit
            InstructionType::CLI => { self.sr.clear_bit(INT_DISABLE_BIT); }

            // Clear Overflow Flag
            InstructionType::CLV => { self.sr.clear_bit(OVERFLOW_BIT); }

            // Compare Memory with Accumulator
            InstructionType::CMP => {
                let operand = self.get_operand(instruction);
                let result = self.a.overflowing_sub(operand).0;
                if self.a >= operand {
                    self.sr.set_bit(CARRY_BIT);
                }
                self.set_sr_nz(result)
            }

            // Compare Memory with Accumulator
            InstructionType::CPX => {
                let operand = self.get_operand(instruction);
                let result = self.x.overflowing_sub(operand).0;
                if self.x >= operand {
                    self.sr.set_bit(CARRY_BIT);
                }
                self.set_sr_nz(result)
            }

            // Compare Memory with Accumulator
            InstructionType::CPY => {
                let operand = self.get_operand(instruction);
                let result = self.y.overflowing_sub(operand).0;
                if self.y >= operand {
                    self.sr.set_bit(CARRY_BIT);
                }
                self.set_sr_nz(result)
            }

            // Decrement Memory by One
            InstructionType::DEC => {
                let operand = self.get_operand(instruction);
                let result = operand.overflowing_sub(1).0;
                self.set_sr_nz(result);
                match &instruction.addr_mode {
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    _ => panic!("Illegal addressing mode for DEC!")
                }
            }

            // Decrement Index X by One
            InstructionType::DEX => {
                self.x = self.x.overflowing_sub(1).0;
                self.set_sr_nz(self.x);
            }

            // Decrement Index Y by One
            InstructionType::DEY => {
                self.y = self.y.overflowing_sub(1).0;
                self.set_sr_nz(self.y);
            }

            // Exclusive-OR Memory with Accumulator
            InstructionType::EOR => {
                let operand = self.get_operand(instruction);
                self.a ^= operand;
                self.set_sr_nz(self.a);
            }

            // Increment Memory by One
            InstructionType::INC => {
                let operand = self.get_operand(instruction);
                let result = operand.overflowing_add(1).0;
                self.set_sr_nz(result);
                match &instruction.addr_mode {
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = result;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = result;
                    }
                    _ => panic!("Illegal addressing mode for INC!")
                }
            }

            // Increment Index X by One
            InstructionType::INX => {
                self.x = self.x.overflowing_add(1).0;
                self.set_sr_nz(self.x);
            }

            // Increment Index Y by One
            InstructionType::INY => {
                self.y = self.y.overflowing_add(1).0;
                self.set_sr_nz(self.y);
            }

            // Jump to New Location
            InstructionType::JMP => {
                let jump_addr = match &instruction.addr_mode {
                    AddrMode::Abs(addr) => *addr,
                    AddrMode::Ind(addr) => {
                        let low_byte = self.ram[*addr as usize];
                        let high_byte = self.ram[*addr as usize + 1];
                        (high_byte as u16) << 8 | (low_byte as u16)
                    }
                    _ => panic!("Illegal addressing mode for JMP!")
                };
                self.pc = jump_addr;
                self.pc -= instruction.machine_code.len() as u16; // compensate for normal pc adjustment
            }

            // Jump to New Location Saving Return Address
            InstructionType::JSR => {
                if let AddrMode::Abs(addr) = &instruction.addr_mode {
                    self.stack_push(self.pc+2);
                    self.pc = *addr;
                    self.pc -= instruction.machine_code.len() as u16; // compensate for normal pc adjustment
                }
            }

            // Store Accumulator in Memory
            InstructionType::STA => {
                match &instruction.addr_mode {
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = self.a;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = self.a;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = self.a;
                    }
                    AddrMode::AbsX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = self.a;
                    }
                    AddrMode::AbsY(addr) => {
                        self.ram[*addr as usize + self.y as usize] = self.a;
                    }
                    AddrMode::XInd(addr) => {
                        let indirect = self.ram[(*addr + self.x) as usize] as usize;
                        self.ram[indirect] = self.a
                    }
                    AddrMode::IndY(addr) => {
                        let indirect = self.ram[*addr as usize] as usize;
                        self.ram[indirect + self.y as usize] = self.a
                    }
                    _ => panic!("Illegal addressing mode for STA!")
                }
            }

            // Store Index X in Memory
            InstructionType::STX => {
                match &instruction.addr_mode {
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = self.x;
                    }
                    AddrMode::ZpgY(addr) => {
                        self.ram[*addr as usize + self.y as usize] = self.x;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = self.x;
                    }
                    _ => panic!("Illegal addressing mode for STX!")
                }
            }

            // Sore Index Y in Memory
            InstructionType::STY => {
                match &instruction.addr_mode {
                    AddrMode::Zpg(addr) => {
                        self.ram[*addr as usize] = self.y;
                    }
                    AddrMode::ZpgX(addr) => {
                        self.ram[*addr as usize + self.x as usize] = self.y;
                    }
                    AddrMode::Abs(addr) => {
                        self.ram[*addr as usize] = self.y;
                    }
                    _ => panic!("Illegal addressing mode for STX!")
                }
            }

            // Transfer Accumulator to Index X
            InstructionType::TAX => {
                self.x = self.a;
                self.set_sr_nz(self.x);
            }

            // Transfer Accumulator to Index Y
            InstructionType::TAY => {
                self.y = self.a;
                self.set_sr_nz(self.y);
            }

            // Transfer Stack Pointer to Index X
            InstructionType::TSX => {
                self.x = self.sp;
                self.set_sr_nz(self.x);
            }

            // Transfer Index X to Accumulator
            InstructionType::TXA => {
                self.a = self.x;
                self.set_sr_nz(self.a);
            }

            // Transfer Index X to Stack Register
            InstructionType::TXS => { self.sp = self.x; }

            // Transfer Index X to Stack Register
            InstructionType::TYA => {
                self.a = self.y;
                self.set_sr_nz(self.a);
            }

            _ => panic!("Emulation for the instruction not yet implemented!\n  {:?}", instruction)
        }

        // addition is wrapping since some branch instructions rely on this behavior
        self.pc = self.pc.wrapping_add(instruction.machine_code.len() as u16);
    }

    // stack manipulation
    fn stack_push_byte(&mut self, byte: u8) {
        self.ram[0x0100 as usize + self.sp as usize] = byte;
        self.sp = (Wrapping(self.sp) - Wrapping(1u8)).0;
    }
    // pop byte from stack
    fn stack_pop_byte(&mut self) -> u8 {
        self.sp = (Wrapping(self.sp) + Wrapping(1u8)).0;
        self.ram[0x0100 as usize + self.sp as usize]
    }
    // push u16 to stack (high byte first)
    fn stack_push(&mut self, value: u16) {
        self.stack_push_byte((value >> 8) as u8);
        self.stack_push_byte((value & 0xff) as u8);
    }
    // pop u16 from stack
    fn stack_pop(&mut self) -> u16 {
        let low_byte = self.stack_pop_byte();
        let high_byte = self.stack_pop_byte();
        (high_byte as u16) << 8 | (low_byte as u16)
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
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.a, self.x, self.y, self.sr, self.sp
        )
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{BitOps, CPU, CARRY_BIT, OVERFLOW_BIT};

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
    fn adc_carry_flag() {
        let mut cpu = CPU::init();

        cpu.load_hexdump("./hexdumps/tests/adc_carry_test.txt").unwrap();
        cpu.pc = 0x0600;

        // CLC, LDA #$FF, ADC #$01
        // sum: 1111_1111 + 0000_0001 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // CLC, LDA #$80, ADC #$80
        // sum: 1000_000 + 1000_0000 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // CLC, LDA #$C0, ADC #$40
        // sum: 1100_000 + 0100_0000 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // SEC, LDA #$fe, ADC #$01
        // sum: 1111_1110 + 0000_0001 (should carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1);
        assert_eq!(cpu.a, 0x00);

        // CLC, LDA #$80, ADC #$40
        // sum: 1000_000 + 0100_0000 (should not carry)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 0);
        assert_eq!(cpu.a, 0xc0);
    }

    #[test]
    fn adc_overflow_flag() {
        let mut cpu = CPU::init();

        cpu.load_hexdump("./hexdumps/tests/adc_overflow_test.txt").unwrap();
        cpu.pc = 0x0600;

        // CLC, LDA #$50, ADC #$50
        // 80 + 80 = 160 > 127 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0xa0);

        // CLC, LDA #$7f, ADC #$01
        // 127 + 1 = 128 > 127 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0x80);

        // SEC, LDA #$7f, ADC #$00
        // 127 + 0 + 1 = 128 > 127 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0x80);

        // CLC, LDA #$7e, ADC #$00
        // 126 + 1 = 127 <= 127 (should not set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 0);
        assert_eq!(cpu.a, 0x7f);
    }

    #[test]
    fn sbc_carry_flag() {
        let mut cpu = CPU::init();

        cpu.load_hexdump("./hexdumps/tests/sbc_overflow_test.txt").unwrap();
        cpu.pc = 0x0600;

        // SEC, LDA #$50, SBC #$b0
        // 80 - -80 = -96 (should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0xa0);
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1u8 - 1);

        // SEC, LDA #$d0, SBC #$70
        // -48 - 112 = 96 >(should set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 1);
        assert_eq!(cpu.a, 0x60);
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1u8 - 0);

        // SEC, LDA #$50, SBC #$f0
        // 80 - -16 = 96 >(should not set overflow)
        for _i in 0..3 {
            cpu.tick().unwrap();
        }
        assert_eq!(cpu.sr.get_bit(OVERFLOW_BIT), 0);
        assert_eq!(cpu.a, 0x60);
        assert_eq!(cpu.sr.get_bit(CARRY_BIT), 1u8 - 1);
    }

    #[test]
    fn functional_test() {
        // TODO: Add asserts
        let mut cpu = CPU::init();

        cpu.load_ines("./hexdumps/tests/nestest.nes").unwrap();
        cpu.pc = 0xc000;
        loop {
            cpu.tick();
        }
    }
}