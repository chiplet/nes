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

// container for the data stored in an instruction
// TODO: format as disassembly and memory dump (String)
#[derive(Debug)]
pub struct InstructionData {
    pub opcode: u8,
    pub addr_mode: AddrMode,
}
impl InstructionData {
    // instruction size in bytes
    // TODO: test
    pub fn size(&self) -> u16 {
        1 + match &self.addr_mode {
            AddrMode::A => 0,
            AddrMode::Abs(_) => 2,
            AddrMode::AbsX(_) => 2,
            AddrMode::AbsY(_) => 2,
            AddrMode::Imm(_) => 1,
            AddrMode::Impl => 0,
            AddrMode::Ind(_) => 2,
            AddrMode::XInd(_) => 1,
            AddrMode::IndY(_) => 1,
            AddrMode::Rel(_) => 1,
            AddrMode::Zpg(_) => 1,
            AddrMode::ZpgX(_) => 1,
            AddrMode::ZpgY(_) => 1,
        }
    }
}

// instruction types
#[derive(Debug)]
pub enum Instruction {
    ADC(InstructionData),
    AND(InstructionData),
    ASL(InstructionData),
    BCC(InstructionData),
    BCS(InstructionData),
    BEQ(InstructionData),
    BIT(InstructionData),
    BMI(InstructionData),
    BNE(InstructionData),
    BPL(InstructionData),
    BRK(InstructionData),
    BVC(InstructionData),
    BVS(InstructionData),
    CLC(InstructionData),
    CLD(InstructionData),
    CLI(InstructionData),
    CLV(InstructionData),
    CMP(InstructionData),
    CPX(InstructionData),
    CPY(InstructionData),
    DEC(InstructionData),
    DEX(InstructionData),
    DEY(InstructionData),
    EOR(InstructionData),
    INC(InstructionData),
    INX(InstructionData),
    INY(InstructionData),
    JMP(InstructionData),
    JSR(InstructionData),
    LDA(InstructionData),
    LDX(InstructionData),
    LDY(InstructionData),
    LSR(InstructionData),
    NOP(InstructionData),
    ORA(InstructionData),
    PHA(InstructionData),
    PHP(InstructionData),
    PLA(InstructionData),
    PLP(InstructionData),
    ROL(InstructionData),
    ROR(InstructionData),
    RTI(InstructionData),
    RTS(InstructionData),
    SBC(InstructionData),
    SEC(InstructionData),
    SED(InstructionData),
    SEI(InstructionData),
    STA(InstructionData),
    STX(InstructionData),
    STY(InstructionData),
    TAX(InstructionData),
    TAY(InstructionData),
    TSX(InstructionData),
    TXA(InstructionData),
    TXS(InstructionData),
    TYA(InstructionData),
}


pub struct InstructionName {
    mnemonic: &'static str,
    description: &'static str,
}

/** decoding helpers **/
// assuming opcode is stored at bytes[0]
// try to get u8 argument from the slice
fn get_u8(bytes: &[u8]) -> Result<u8, &str> {
    match bytes.get(1) {
        Some(value) => Ok(*value),
        None => Err("Could not extract u8 operand")
    }
}
// try to get u8 argument from the slice at given index
fn get_u8_at(bytes: &[u8], index: usize) -> Result<u8, &str> {
    match bytes.get(index) {
        Some(value) => Ok(*value),
        None => Err("Could not extract u8 operand")
    }
}
// try to get u16 (little endian) argument from the slice
fn get_u16(bytes: &[u8]) -> Result<u16, &str> {
    let lower_byte = get_u8_at(bytes, 1)?;
    let higher_byte = get_u8_at(bytes, 2)?;
    let result = (higher_byte as u16) << 8 | (lower_byte as u16);
    Ok(result)
}

impl Instruction {
    // decode the first instruction from a u8 slice
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 0 {
            return Err("No bytes to decode!".to_string());
        }

        // parse opcode to Instruction with this MEGA match expression
        // the match arms have been generated with `sripts/parse_instructions.py`
        match bytes[0] {
            0x69 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0x65 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x75 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x6D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x7D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x79 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0x61 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0x71 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ADC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0x29 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0x25 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x35 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x2D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x3D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x39 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0x21 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0x31 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::AND(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0x0A => {
                Ok(Instruction::ASL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::A,
                }))
            }
            0x06 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ASL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x16 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ASL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x0E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ASL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x1E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ASL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x90 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BCC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0xB0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BCS(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0xF0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BEQ(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0x24 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BIT(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x2C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::BIT(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x30 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BMI(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0xD0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BNE(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0x10 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BPL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0x00 => {
                Ok(Instruction::BRK(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x50 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BVC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0x70 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::BVC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Rel(arg as i8),
                }))
            }
            0x18 => {
                Ok(Instruction::CLC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xD8 => {
                Ok(Instruction::CLD(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x58 => {
                Ok(Instruction::CLI(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xB8 => {
                Ok(Instruction::CLV(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xC9 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0xC5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xD5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0xCD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xDD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0xD9 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0xC1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0xD1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0xE0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CPX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0xE4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CPX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xEC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::CPX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xC0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CPY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0xC4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::CPY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xCC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::CPY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xC6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::DEC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xD6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::DEC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0xCE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::DEC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xDE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::DEC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0xCA => {
                Ok(Instruction::DEC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x88 => {
                Ok(Instruction::DEC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x49 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0x45 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x55 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x4D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x5D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x59 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0x41 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0x51 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::EOR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0xE6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::INC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xF6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::INC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0xEE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::INC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xFE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::INC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0xE8 => {
                Ok(Instruction::INX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xC8 => {
                Ok(Instruction::INY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x4C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::JMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x6C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::JMP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Ind(arg),
                }))
            }
            0x20 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::JSR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xA9 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0xA5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xB5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0xAD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xBD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0xB9 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0xA1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0xB1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0xA2 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0xA6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xB6 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgY(arg),
                }))
            }
            0xAE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LDX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xBE => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LDX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0xA0 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0xA4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xB4 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LDY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0xAC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LDY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xBC => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LDY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x4A => {
                Ok(Instruction::LSR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::A,
                }))
            }
            0x46 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LSR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x56 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::LSR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x4E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LSR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x5E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::LSR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0xEA => {
                Ok(Instruction::NOP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x09 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0x05 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x15 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x0D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x1D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x19 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0x01 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0x11 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ORA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0x48 => {
                Ok(Instruction::PHA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x08 => {
                Ok(Instruction::PHP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x68 => {
                Ok(Instruction::PLA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x28 => {
                Ok(Instruction::PLP(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x2A => {
                Ok(Instruction::ROL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::A,
                }))
            }
            0x26 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ROL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x36 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ROL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x2E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ROL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x3E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ROL(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x6A => {
                Ok(Instruction::ROR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::A,
                }))
            }
            0x66 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ROR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x76 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::ROR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x6E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ROR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x7E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::ROR(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x40 => {
                Ok(Instruction::RTI(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x60 => {
                Ok(Instruction::RTS(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xE9 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Imm(arg),
                }))
            }
            0xE5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0xF5 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0xED => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xFD => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0xF9 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0xE1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0xF1 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::SBC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0x38 => {
                Ok(Instruction::SEC(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xF8 => {
                Ok(Instruction::SED(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x78 => {
                Ok(Instruction::SEI(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x85 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x95 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x8D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::STA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x9D => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::STA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsX(arg),
                }))
            }
            0x99 => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::STA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::AbsY(arg),
                }))
            }
            0x81 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::XInd(arg),
                }))
            }
            0x91 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::IndY(arg),
                }))
            }
            0x86 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x96 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgY(arg),
                }))
            }
            0x8E => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::STX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0x84 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Zpg(arg),
                }))
            }
            0x94 => {
                let arg = get_u8(bytes)?;
                Ok(Instruction::STY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::ZpgX(arg),
                }))
            }
            0x8C => {
                let arg = get_u16(bytes)?;
                Ok(Instruction::STY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Abs(arg),
                }))
            }
            0xAA => {
                Ok(Instruction::TAX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xA8 => {
                Ok(Instruction::TAY(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0xBA => {
                Ok(Instruction::TSX(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x8A => {
                Ok(Instruction::TXA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x9A => {
                Ok(Instruction::TXS(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            0x98 => {
                Ok(Instruction::TYA(InstructionData {
                    opcode: bytes[0],
                    addr_mode: AddrMode::Impl,
                }))
            }
            _ => Err(format!("Decoding not implemented for opcode: 0x{:02x}", bytes[0]))
        }
    }
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
