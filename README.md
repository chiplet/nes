# nes
This project is my attempt to implement a Nintendo Entertainment System (NES) emulator using the Rust programming language.

## Goals
**Goals**

The main goal of the project is to act as a playground for learning advanced Rust language features by applying them in practice. The project should be extensive enough to facilitate learning about Rust's module system and how to properly organize code in a large project.


**Non-goals**

This emulator does NOT attempt:
* To be the most accurate emulator
* To be the highest performance emulator
* To have stable implementation or interface

## Building the Project
Make sure you have installed the Rust compiler and toolchain. This can be done most conveniently with the [rustup](https://rustup.rs/) tool.

Clone the repository using your preferred method and set it as your working directory, e.g. from the command line:

```
$ git clone https://github.com/chiplet/nes.git
$ cd nes
```

The project uses Rust's `cargo` build system. To install the required dependencies and to build the application with debug flags simply run the command:

`$ cargo build`

The built binary can be run with:

`$ cargo run`

A more optimized release build can be built by compiling with the `--release` flag:

```
$ cargo build --release
$ cargo run --release
```

## Testing

All tests can be run with the command:
```
cargo test --all
```

Many tests load a test program into CPU memory from the `hexdumps/tests/` directory, execute the program, and verify that the CPU is in an expected state.
