# 6502 Assembler

Program made in rust to assemble opcodes for the 6502

Takes a file with the .asm extension and outputs a binary file (`.hex`) that is 32768 bytes long with the assembled code starting at 0x8000

## Usage

Use `cargo run -- --input <path> --output <path>` to run

## Example

`cargo run -- --input assembly/basic_test.asm --output assembly/basic_test.hex`
