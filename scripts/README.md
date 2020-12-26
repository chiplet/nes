This directory contains text files describing the 6502 Instruction Set Architecture and its addressing modes.

The information has been mostly adapted from: https://www.masswerk.at/6502/6502_instruction_set.html


* `instruction_descriptions.txt` -- Contains 6502 instruction mnemonics and their verbose descriptions
* `instructions.txt` -- Contains information about the 6502 instructions and their addressing modes
* `parse_instructions.py` -- A script that generated Rust match arms for decoding opcodes to Instruction structs.
* `parse_names.py` -- A script that generates Rust match arms for decoding opcodes to a textual representation.
