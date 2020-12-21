# helper script for generating textual representations for instructions

def readlines(filepath):
    with open(filepath, 'r') as fd:
        return [line.strip() for line in fd.readlines()]

def main():
    descriptions = dict([line.split("  ") for line in readlines('instruction_descriptions.txt')])

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

        print(f"0x{opcode} => {{ InstructionName {{ mnemonic: \"{mnemonic}\", description: \"{descriptions[mnemonic]}\", }}}}")

if __name__ == '__main__':
    main()
