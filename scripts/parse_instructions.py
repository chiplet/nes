# helper script for generating rust code for instruction decoding

def readlines(filepath):
    with open(filepath, 'r') as fd:
        return [line.strip() for line in fd.readlines()]

# map instructions.txt file format to rust code addressing mode names
namemap = {
    "accumulator": "A",
    "absolute": "Abs",
    "absolute,X": "AbsX",
    "absolute,Y": "AbsY",
    "immediate": "Imm",
    "implied": "Impl",
    "indirect": "Ind",
    "(indirect,X)": "XInd",
    "(indirect),Y": "IndY",
    "relative": "Rel",
    "zeropage": "Zpg",
    "zeropage,X": "ZpgX",
    "zeropage,Y": "ZpgY",
}

def main():
    lines = readlines('instructions.txt')
    for line in lines[1:]:
        addr_mode = None
        mnemonic = None
        operand = None
        opcode = None
        size = None
        cycles = None

        if len(line.split()) == 6:
            addr_mode, mnemonic, operand, opcode, size, cycles = line.split()
        elif len(line.split()) == 5:
            addr_mode, mnemonic, opcode, size, cycles = line.split()
        else:
            raise NotImplementedException("unhandled line format")

        # print match arms for Rust instruction decoding
        print(f"0x{opcode} => {{")

        # operands
        if addr_mode in ["immediate", "(indirect,X)", "(indirect),Y", "zeropage", "zeropage,X", "zeropage,Y"]:
            print(f"    let arg = get_u8(bytes)?;")
            print(f"    Ok(Instruction::{mnemonic}(InstructionData {{")
            print(f"        opcode: bytes[0],")
            print(f"        addr_mode: AddrMode::{namemap[addr_mode]}(arg),")
        elif addr_mode in ["accumulator", "implied"]:
            print(f"    Ok(Instruction::{mnemonic}(InstructionData {{")
            print(f"        opcode: bytes[0],")
            print(f"        addr_mode: AddrMode::{namemap[addr_mode]},")
        elif addr_mode in ["absolute", "absolute,X", "absolute,Y", "indirect"]:
            print(f"    let arg = get_u16(bytes)?;")
            print(f"    Ok(Instruction::{mnemonic}(InstructionData {{")
            print(f"        opcode: bytes[0],")
            print(f"        addr_mode: AddrMode::{namemap[addr_mode]}(arg),")
        elif addr_mode == "relative":
            print(f"    let arg = get_u8(bytes)?;")
            print(f"    Ok(Instruction::{mnemonic}(InstructionData {{")
            print(f"        opcode: bytes[0],")
            print(f"        addr_mode: AddrMode::{namemap[addr_mode]}(arg as i8),")

        print("    }))\n}")

        # 0x69 => {
        #     let arg = get_u8(bytes)?;
        #     Ok(Instruction::ADC(InstructionData {
        #         opcode: bytes[0],
        #         addr_mode: AddrMode::Imm(arg)
        #     }))
        # }


if __name__ == '__main__':
    main()
