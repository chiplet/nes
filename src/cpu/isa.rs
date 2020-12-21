/** Abstractions for the 6502 CPU instruction set **/
use std::fmt;

// instruction addressing mode with the associated argument (memory address / offset)
#[derive(Debug)]
pub enum AddrMode {
    A,              // accumulator
    Abs(u16),       // absolute
    AbsX(u16),      // absolute, X-indexed
    AbsY(u16),      // absolute, Y-indexed
    Imm(u8),        // immediate
    Impl,           // implied
    Ind(u16),       // indirect
    XInd(u8),       // X-indexed, indirect
    IndY(u8),       // indirect, Y-indexed
    Rel(i8),        // relative
    Zpg(u8),        // zeropage
    ZpgX(u8),       // zeropage, X-indexed
    ZpgY(u8),       // zeropage, Y-indexed
}

#[derive(Debug)]
pub enum InstructionType {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP, CPX,
    CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA,
    PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,
}

#[derive(Debug)]
pub struct InstructionName {
    pub mnemonic: &'static str,
    pub description: &'static str,
}
impl InstructionName {
    fn from(opcode: u8) -> Self {
        // match arms were generated with `scripts/parse_names.py`
        match opcode {
            0x69 => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x65 => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x75 => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x6D => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x7D => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x79 => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x61 => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x71 => { InstructionName { mnemonic: "ADC", description: "Add Memory to Accumulator with Carry", }}
            0x29 => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x25 => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x35 => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x2D => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x3D => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x39 => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x21 => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x31 => { InstructionName { mnemonic: "AND", description: "AND Memory with Accumulator", }}
            0x0A => { InstructionName { mnemonic: "ASL", description: "Shift Left One Bit (Memory or Accumulator)", }}
            0x06 => { InstructionName { mnemonic: "ASL", description: "Shift Left One Bit (Memory or Accumulator)", }}
            0x16 => { InstructionName { mnemonic: "ASL", description: "Shift Left One Bit (Memory or Accumulator)", }}
            0x0E => { InstructionName { mnemonic: "ASL", description: "Shift Left One Bit (Memory or Accumulator)", }}
            0x1E => { InstructionName { mnemonic: "ASL", description: "Shift Left One Bit (Memory or Accumulator)", }}
            0x90 => { InstructionName { mnemonic: "BCC", description: "Branch on Carry Clear", }}
            0xB0 => { InstructionName { mnemonic: "BCS", description: "Branch on Carry Set", }}
            0xF0 => { InstructionName { mnemonic: "BEQ", description: "Branch on Result Zero", }}
            0x24 => { InstructionName { mnemonic: "BIT", description: "Test Bits in Memory with Accumulator", }}
            0x2C => { InstructionName { mnemonic: "BIT", description: "Test Bits in Memory with Accumulator", }}
            0x30 => { InstructionName { mnemonic: "BMI", description: "Branch on Result Minus", }}
            0xD0 => { InstructionName { mnemonic: "BNE", description: "Branch on Result not Zero", }}
            0x10 => { InstructionName { mnemonic: "BPL", description: "Branch on Result Plus", }}
            0x00 => { InstructionName { mnemonic: "BRK", description: "Force Break", }}
            0x50 => { InstructionName { mnemonic: "BVC", description: "Branch on Overflow Clear", }}
            0x70 => { InstructionName { mnemonic: "BVC", description: "Branch on Overflow Clear", }}
            0x18 => { InstructionName { mnemonic: "CLC", description: "Clear Carry Flag", }}
            0xD8 => { InstructionName { mnemonic: "CLD", description: "Clear Decimal Mode", }}
            0x58 => { InstructionName { mnemonic: "CLI", description: "Clear Interrupt Disable Bit", }}
            0xB8 => { InstructionName { mnemonic: "CLV", description: "Clear Overflow Flag", }}
            0xC9 => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xC5 => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xD5 => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xCD => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xDD => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xD9 => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xC1 => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xD1 => { InstructionName { mnemonic: "CMP", description: "Compare Memory with Accumulator", }}
            0xE0 => { InstructionName { mnemonic: "CPX", description: "Compare Memory and Index X", }}
            0xE4 => { InstructionName { mnemonic: "CPX", description: "Compare Memory and Index X", }}
            0xEC => { InstructionName { mnemonic: "CPX", description: "Compare Memory and Index X", }}
            0xC0 => { InstructionName { mnemonic: "CPY", description: "Compare Memory and Index Y", }}
            0xC4 => { InstructionName { mnemonic: "CPY", description: "Compare Memory and Index Y", }}
            0xCC => { InstructionName { mnemonic: "CPY", description: "Compare Memory and Index Y", }}
            0xC6 => { InstructionName { mnemonic: "DEC", description: "Decrement Memory by One", }}
            0xD6 => { InstructionName { mnemonic: "DEC", description: "Decrement Memory by One", }}
            0xCE => { InstructionName { mnemonic: "DEC", description: "Decrement Memory by One", }}
            0xDE => { InstructionName { mnemonic: "DEC", description: "Decrement Memory by One", }}
            0xCA => { InstructionName { mnemonic: "DEC", description: "Decrement Memory by One", }}
            0x88 => { InstructionName { mnemonic: "DEC", description: "Decrement Memory by One", }}
            0x49 => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0x45 => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0x55 => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0x4D => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0x5D => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0x59 => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0x41 => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0x51 => { InstructionName { mnemonic: "EOR", description: "Exclusive-OR Memory with Accumulator", }}
            0xE6 => { InstructionName { mnemonic: "INC", description: "Increment Memory by One", }}
            0xF6 => { InstructionName { mnemonic: "INC", description: "Increment Memory by One", }}
            0xEE => { InstructionName { mnemonic: "INC", description: "Increment Memory by One", }}
            0xFE => { InstructionName { mnemonic: "INC", description: "Increment Memory by One", }}
            0xE8 => { InstructionName { mnemonic: "INX", description: "Increment Index X by One", }}
            0xC8 => { InstructionName { mnemonic: "INY", description: "Increment Index Y by One", }}
            0x4C => { InstructionName { mnemonic: "JMP", description: "Jump to New Location", }}
            0x6C => { InstructionName { mnemonic: "JMP", description: "Jump to New Location", }}
            0x20 => { InstructionName { mnemonic: "JSR", description: "Jump to New Location Saving Return Address", }}
            0xA9 => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xA5 => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xB5 => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xAD => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xBD => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xB9 => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xA1 => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xB1 => { InstructionName { mnemonic: "LDA", description: "Load Accumulator with Memory", }}
            0xA2 => { InstructionName { mnemonic: "LDX", description: "Load Index X with Memory", }}
            0xA6 => { InstructionName { mnemonic: "LDX", description: "Load Index X with Memory", }}
            0xB6 => { InstructionName { mnemonic: "LDX", description: "Load Index X with Memory", }}
            0xAE => { InstructionName { mnemonic: "LDX", description: "Load Index X with Memory", }}
            0xBE => { InstructionName { mnemonic: "LDX", description: "Load Index X with Memory", }}
            0xA0 => { InstructionName { mnemonic: "LDY", description: "Load Index Y with Memory", }}
            0xA4 => { InstructionName { mnemonic: "LDY", description: "Load Index Y with Memory", }}
            0xB4 => { InstructionName { mnemonic: "LDY", description: "Load Index Y with Memory", }}
            0xAC => { InstructionName { mnemonic: "LDY", description: "Load Index Y with Memory", }}
            0xBC => { InstructionName { mnemonic: "LDY", description: "Load Index Y with Memory", }}
            0x4A => { InstructionName { mnemonic: "LSR", description: "Shift One Bit Right (Memory or Accumulator)", }}
            0x46 => { InstructionName { mnemonic: "LSR", description: "Shift One Bit Right (Memory or Accumulator)", }}
            0x56 => { InstructionName { mnemonic: "LSR", description: "Shift One Bit Right (Memory or Accumulator)", }}
            0x4E => { InstructionName { mnemonic: "LSR", description: "Shift One Bit Right (Memory or Accumulator)", }}
            0x5E => { InstructionName { mnemonic: "LSR", description: "Shift One Bit Right (Memory or Accumulator)", }}
            0xEA => { InstructionName { mnemonic: "NOP", description: "No Operation", }}
            0x09 => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x05 => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x15 => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x0D => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x1D => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x19 => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x01 => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x11 => { InstructionName { mnemonic: "ORA", description: "OR Memory with Accumulator", }}
            0x48 => { InstructionName { mnemonic: "PHA", description: "Push Accumulator on Stack", }}
            0x08 => { InstructionName { mnemonic: "PHP", description: "Push Processor Status on Stack", }}
            0x68 => { InstructionName { mnemonic: "PLA", description: "Pull Accumulator from Stack", }}
            0x28 => { InstructionName { mnemonic: "PLP", description: "Pull Processor Status from Stack", }}
            0x2A => { InstructionName { mnemonic: "ROL", description: "Rotate One Bit Left (Memory or Accumulator)", }}
            0x26 => { InstructionName { mnemonic: "ROL", description: "Rotate One Bit Left (Memory or Accumulator)", }}
            0x36 => { InstructionName { mnemonic: "ROL", description: "Rotate One Bit Left (Memory or Accumulator)", }}
            0x2E => { InstructionName { mnemonic: "ROL", description: "Rotate One Bit Left (Memory or Accumulator)", }}
            0x3E => { InstructionName { mnemonic: "ROL", description: "Rotate One Bit Left (Memory or Accumulator)", }}
            0x6A => { InstructionName { mnemonic: "ROR", description: "Rotate One Bit Right (Memory or Accumulator)", }}
            0x66 => { InstructionName { mnemonic: "ROR", description: "Rotate One Bit Right (Memory or Accumulator)", }}
            0x76 => { InstructionName { mnemonic: "ROR", description: "Rotate One Bit Right (Memory or Accumulator)", }}
            0x6E => { InstructionName { mnemonic: "ROR", description: "Rotate One Bit Right (Memory or Accumulator)", }}
            0x7E => { InstructionName { mnemonic: "ROR", description: "Rotate One Bit Right (Memory or Accumulator)", }}
            0x40 => { InstructionName { mnemonic: "RTI", description: "Return from Interrupt", }}
            0x60 => { InstructionName { mnemonic: "RTS", description: "Return from Subroutine", }}
            0xE9 => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0xE5 => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0xF5 => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0xED => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0xFD => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0xF9 => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0xE1 => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0xF1 => { InstructionName { mnemonic: "SBC", description: "Subtract Memory from Accumulator with Borrow", }}
            0x38 => { InstructionName { mnemonic: "SEC", description: "Set Carry Flag", }}
            0xF8 => { InstructionName { mnemonic: "SED", description: "Set Decimal Flag", }}
            0x78 => { InstructionName { mnemonic: "SEI", description: "Set Interrupt Disable Status", }}
            0x85 => { InstructionName { mnemonic: "STA", description: "Store Accumulator in Memory", }}
            0x95 => { InstructionName { mnemonic: "STA", description: "Store Accumulator in Memory", }}
            0x8D => { InstructionName { mnemonic: "STA", description: "Store Accumulator in Memory", }}
            0x9D => { InstructionName { mnemonic: "STA", description: "Store Accumulator in Memory", }}
            0x99 => { InstructionName { mnemonic: "STA", description: "Store Accumulator in Memory", }}
            0x81 => { InstructionName { mnemonic: "STA", description: "Store Accumulator in Memory", }}
            0x91 => { InstructionName { mnemonic: "STA", description: "Store Accumulator in Memory", }}
            0x86 => { InstructionName { mnemonic: "STX", description: "Store Index X in Memory", }}
            0x96 => { InstructionName { mnemonic: "STX", description: "Store Index X in Memory", }}
            0x8E => { InstructionName { mnemonic: "STX", description: "Store Index X in Memory", }}
            0x84 => { InstructionName { mnemonic: "STY", description: "Store Index Y in Memory", }}
            0x94 => { InstructionName { mnemonic: "STY", description: "Store Index Y in Memory", }}
            0x8C => { InstructionName { mnemonic: "STY", description: "Store Index Y in Memory", }}
            0xAA => { InstructionName { mnemonic: "TAX", description: "Transfer Accumulator to Index X", }}
            0xA8 => { InstructionName { mnemonic: "TAY", description: "Transfer Accumulator to Index Y", }}
            0xBA => { InstructionName { mnemonic: "TSX", description: "Transfer Stack Pointer to Index X", }}
            0x8A => { InstructionName { mnemonic: "TXA", description: "Transfer Index X to Accumulator", }}
            0x9A => { InstructionName { mnemonic: "TXS", description: "Transfer Index X to Stack Register", }}
            0x98 => { InstructionName { mnemonic: "TYA", description: "Transfer Index Y to Accumulator", }}
            _ => panic!("Illegal opcode: {}", opcode)
        }
    }
}

// Instruction to be executed by the processor and related useful information
// TODO: format as disassembly and memory dump (String)
#[derive(Debug)]
pub struct Instruction {
    pub machine_code: Vec<u8>,
    pub ins_type: InstructionType,
    pub addr_mode: AddrMode,
    pub name: InstructionName,
}
impl Instruction {
    // decode single instruction from byte slice
    pub fn from(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 0 {
            return Err("No bytes to decode!".to_string());
        }

        // parse opcode to Instruction with this MEGA match expression
        // the match arms have been generated with `sripts/parse_instructions.py`
        match bytes[0] {
            0x69 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0x69),
                })
            }
            0x65 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x65),
                })
            }
            0x75 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x75),
                })
            }
            0x6D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x6D),
                })
            }
            0x7D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x7D),
                })
            }
            0x79 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0x79),
                })
            }
            0x61 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0x61),
                })
            }
            0x71 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ADC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0x71),
                })
            }
            0x29 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0x29),
                })
            }
            0x25 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x25),
                })
            }
            0x35 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x35),
                })
            }
            0x2D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x2D),
                })
            }
            0x3D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x3D),
                })
            }
            0x39 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0x39),
                })
            }
            0x21 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0x21),
                })
            }
            0x31 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::AND,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0x31),
                })
            }
            0x0A => {
                Ok(Instruction {
                    ins_type: InstructionType::ASL,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::A,
                    name: InstructionName::from(0x0A),
                })
            }
            0x06 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ASL,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x06),
                })
            }
            0x16 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ASL,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x16),
                })
            }
            0x0E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ASL,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x0E),
                })
            }
            0x1E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ASL,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x1E),
                })
            }
            0x90 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BCC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0x90),
                })
            }
            0xB0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BCS,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0xB0),
                })
            }
            0xF0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BEQ,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0xF0),
                })
            }
            0x24 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BIT,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x24),
                })
            }
            0x2C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BIT,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x2C),
                })
            }
            0x30 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BMI,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0x30),
                })
            }
            0xD0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BNE,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0xD0),
                })
            }
            0x10 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BPL,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0x10),
                })
            }
            0x00 => {
                Ok(Instruction {
                    ins_type: InstructionType::BRK,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x00),
                })
            }
            0x50 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BVC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0x50),
                })
            }
            0x70 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::BVC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Rel(arg as i8),
                    name: InstructionName::from(0x70),
                })
            }
            0x18 => {
                Ok(Instruction {
                    ins_type: InstructionType::CLC,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x18),
                })
            }
            0xD8 => {
                Ok(Instruction {
                    ins_type: InstructionType::CLD,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xD8),
                })
            }
            0x58 => {
                Ok(Instruction {
                    ins_type: InstructionType::CLI,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x58),
                })
            }
            0xB8 => {
                Ok(Instruction {
                    ins_type: InstructionType::CLV,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xB8),
                })
            }
            0xC9 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0xC9),
                })
            }
            0xC5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xC5),
                })
            }
            0xD5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0xD5),
                })
            }
            0xCD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xCD),
                })
            }
            0xDD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0xDD),
                })
            }
            0xD9 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0xD9),
                })
            }
            0xC1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0xC1),
                })
            }
            0xD1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CMP,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0xD1),
                })
            }
            0xE0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CPX,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0xE0),
                })
            }
            0xE4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CPX,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xE4),
                })
            }
            0xEC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CPX,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xEC),
                })
            }
            0xC0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CPY,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0xC0),
                })
            }
            0xC4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CPY,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xC4),
                })
            }
            0xCC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::CPY,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xCC),
                })
            }
            0xC6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::DEC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xC6),
                })
            }
            0xD6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::DEC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0xD6),
                })
            }
            0xCE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::DEC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xCE),
                })
            }
            0xDE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::DEC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0xDE),
                })
            }
            0xCA => {
                Ok(Instruction {
                    ins_type: InstructionType::DEC,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xCA),
                })
            }
            0x88 => {
                Ok(Instruction {
                    ins_type: InstructionType::DEC,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x88),
                })
            }
            0x49 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0x49),
                })
            }
            0x45 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x45),
                })
            }
            0x55 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x55),
                })
            }
            0x4D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x4D),
                })
            }
            0x5D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x5D),
                })
            }
            0x59 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0x59),
                })
            }
            0x41 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0x41),
                })
            }
            0x51 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::EOR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0x51),
                })
            }
            0xE6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::INC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xE6),
                })
            }
            0xF6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::INC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0xF6),
                })
            }
            0xEE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::INC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xEE),
                })
            }
            0xFE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::INC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0xFE),
                })
            }
            0xE8 => {
                Ok(Instruction {
                    ins_type: InstructionType::INX,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xE8),
                })
            }
            0xC8 => {
                Ok(Instruction {
                    ins_type: InstructionType::INY,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xC8),
                })
            }
            0x4C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::JMP,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x4C),
                })
            }
            0x6C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::JMP,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Ind(arg),
                    name: InstructionName::from(0x6C),
                })
            }
            0x20 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::JSR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x20),
                })
            }
            0xA9 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0xA9),
                })
            }
            0xA5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xA5),
                })
            }
            0xB5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0xB5),
                })
            }
            0xAD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xAD),
                })
            }
            0xBD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0xBD),
                })
            }
            0xB9 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0xB9),
                })
            }
            0xA1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0xA1),
                })
            }
            0xB1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0xB1),
                })
            }
            0xA2 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDX,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0xA2),
                })
            }
            0xA6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDX,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xA6),
                })
            }
            0xB6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDX,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgY(arg),
                    name: InstructionName::from(0xB6),
                })
            }
            0xAE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDX,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xAE),
                })
            }
            0xBE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDX,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0xBE),
                })
            }
            0xA0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDY,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0xA0),
                })
            }
            0xA4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDY,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xA4),
                })
            }
            0xB4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDY,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0xB4),
                })
            }
            0xAC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDY,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xAC),
                })
            }
            0xBC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LDY,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0xBC),
                })
            }
            0x4A => {
                Ok(Instruction {
                    ins_type: InstructionType::LSR,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::A,
                    name: InstructionName::from(0x4A),
                })
            }
            0x46 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LSR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x46),
                })
            }
            0x56 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LSR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x56),
                })
            }
            0x4E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LSR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x4E),
                })
            }
            0x5E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::LSR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x5E),
                })
            }
            0xEA => {
                Ok(Instruction {
                    ins_type: InstructionType::NOP,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xEA),
                })
            }
            0x09 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0x09),
                })
            }
            0x05 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x05),
                })
            }
            0x15 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x15),
                })
            }
            0x0D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x0D),
                })
            }
            0x1D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x1D),
                })
            }
            0x19 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0x19),
                })
            }
            0x01 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0x01),
                })
            }
            0x11 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ORA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0x11),
                })
            }
            0x48 => {
                Ok(Instruction {
                    ins_type: InstructionType::PHA,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x48),
                })
            }
            0x08 => {
                Ok(Instruction {
                    ins_type: InstructionType::PHP,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x08),
                })
            }
            0x68 => {
                Ok(Instruction {
                    ins_type: InstructionType::PLA,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x68),
                })
            }
            0x28 => {
                Ok(Instruction {
                    ins_type: InstructionType::PLP,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x28),
                })
            }
            0x2A => {
                Ok(Instruction {
                    ins_type: InstructionType::ROL,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::A,
                    name: InstructionName::from(0x2A),
                })
            }
            0x26 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROL,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x26),
                })
            }
            0x36 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROL,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x36),
                })
            }
            0x2E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROL,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x2E),
                })
            }
            0x3E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROL,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x3E),
                })
            }
            0x6A => {
                Ok(Instruction {
                    ins_type: InstructionType::ROR,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::A,
                    name: InstructionName::from(0x6A),
                })
            }
            0x66 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x66),
                })
            }
            0x76 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROR,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x76),
                })
            }
            0x6E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x6E),
                })
            }
            0x7E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::ROR,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x7E),
                })
            }
            0x40 => {
                Ok(Instruction {
                    ins_type: InstructionType::RTI,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x40),
                })
            }
            0x60 => {
                Ok(Instruction {
                    ins_type: InstructionType::RTS,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x60),
                })
            }
            0xE9 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Imm(arg),
                    name: InstructionName::from(0xE9),
                })
            }
            0xE5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0xE5),
                })
            }
            0xF5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0xF5),
                })
            }
            0xED => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0xED),
                })
            }
            0xFD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0xFD),
                })
            }
            0xF9 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0xF9),
                })
            }
            0xE1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0xE1),
                })
            }
            0xF1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::SBC,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0xF1),
                })
            }
            0x38 => {
                Ok(Instruction {
                    ins_type: InstructionType::SEC,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x38),
                })
            }
            0xF8 => {
                Ok(Instruction {
                    ins_type: InstructionType::SED,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xF8),
                })
            }
            0x78 => {
                Ok(Instruction {
                    ins_type: InstructionType::SEI,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x78),
                })
            }
            0x85 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x85),
                })
            }
            0x95 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x95),
                })
            }
            0x8D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x8D),
                })
            }
            0x9D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsX(arg),
                    name: InstructionName::from(0x9D),
                })
            }
            0x99 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STA,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::AbsY(arg),
                    name: InstructionName::from(0x99),
                })
            }
            0x81 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::XInd(arg),
                    name: InstructionName::from(0x81),
                })
            }
            0x91 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STA,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::IndY(arg),
                    name: InstructionName::from(0x91),
                })
            }
            0x86 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STX,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x86),
                })
            }
            0x96 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STX,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgY(arg),
                    name: InstructionName::from(0x96),
                })
            }
            0x8E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STX,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x8E),
                })
            }
            0x84 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STY,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::Zpg(arg),
                    name: InstructionName::from(0x84),
                })
            }
            0x94 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STY,
                    machine_code: bytes.to_vec().into_iter().take(2).collect(),
                    addr_mode: AddrMode::ZpgX(arg),
                    name: InstructionName::from(0x94),
                })
            }
            0x8C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction {
                    ins_type: InstructionType::STY,
                    machine_code: bytes.to_vec().into_iter().take(3).collect(),
                    addr_mode: AddrMode::Abs(arg),
                    name: InstructionName::from(0x8C),
                })
            }
            0xAA => {
                Ok(Instruction {
                    ins_type: InstructionType::TAX,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xAA),
                })
            }
            0xA8 => {
                Ok(Instruction {
                    ins_type: InstructionType::TAY,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xA8),
                })
            }
            0xBA => {
                Ok(Instruction {
                    ins_type: InstructionType::TSX,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0xBA),
                })
            }
            0x8A => {
                Ok(Instruction {
                    ins_type: InstructionType::TXA,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x8A),
                })
            }
            0x9A => {
                Ok(Instruction {
                    ins_type: InstructionType::TXS,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x9A),
                })
            }
            0x98 => {
                Ok(Instruction {
                    ins_type: InstructionType::TYA,
                    machine_code: bytes.to_vec().into_iter().take(1).collect(),
                    addr_mode: AddrMode::Impl,
                    name: InstructionName::from(0x98),
                })
            }
            _ => Err(format!("Decoding not implemented for opcode: ${:02x}", bytes[0]))
        }
    }
}


/** decoding helpers **/
// assuming opcode is stored at bytes[0]
fn get_u8(bytes: &[u8]) -> Result<u8, &str> {
    match bytes.get(1) {
        Some(value) => Ok(*value),
        None => Err("Could not extract u8 operand")
    }
}
fn get_u8_at(bytes: &[u8], index: usize) -> Result<u8, &str> {
    match bytes.get(index) {
        Some(value) => Ok(*value),
        None => Err("Could not extract u8 operand")
    }
}
fn get_u16(bytes: &[u8]) -> Result<u16, &str> {
    let lower_byte = get_u8_at(bytes, 1)?;
    let higher_byte = get_u8_at(bytes, 2)?;
    let result = (higher_byte as u16) << 8 | (lower_byte as u16); // little endian
    Ok(result)
}


#[cfg(test)]
mod test {
    use crate::cpu::isa::{get_u8, get_u8_at, get_u16};

    #[test]
    fn get_u8_valid() {
        let bytes: [u8; 2] = [0x00, 0x23];
        let value = get_u8(&bytes).unwrap();
        assert_eq!(0x23, value);
    }

    #[test]
    #[should_panic]
    fn get_u8_invalid() {
        let bytes: [u8; 1] = [0x00];
        get_u8(&bytes).unwrap();
    }

    #[test]
    fn get_u8_at_valid() {
        let bytes: [u8; 3] = [0x00, 0xcd, 0xab];
        assert_eq!(0x00, get_u8_at(&bytes, 0).unwrap());
        assert_eq!(0xcd, get_u8_at(&bytes, 1).unwrap());
        assert_eq!(0xab, get_u8_at(&bytes, 2).unwrap());
    }

    #[test]
    fn get_u16_valid() {
        let bytes: [u8; 3] = [0x00, 0xcd, 0xab];
        let value = get_u16(&bytes).unwrap();
        assert_eq!(0xabcd, value);
    }
}
