# RISCV HELLO WORLD

- This is a minimal sample of hello world for riscv.

## REQUIREMENTS

- Rust (riscv64gc-unknown-none-elf)
  - `rustup target add riscv64gc-unknown-none-elf``
- qemu-system-riscv64

## HOW TO BUILD

```
$ cargo build --release
```

## HOW TO RUN

```
$ cargo run --release
```
