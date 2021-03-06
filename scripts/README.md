This directory contains text files describing the 6502 Instruction Set Architecture, and python scripts for generating Rust code from them.

The information has been mostly adapted from: https://www.masswerk.at/6502/6502_instruction_set.html


* `instruction_descriptions.txt` -- Contains 6502 instruction mnemonics and their verbose descriptions
* `instructions.txt` -- Contains information about the 6502 instructions and their addressing modes
* `parse_instructions.py` -- Generates Rust match arms for decoding opcodes to Instruction structs.
* `parse_names.py` -- Generates Rust match arms for decoding opcodes to a textual representation.
