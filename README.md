# Shellcide

x86_64 shellcode IDE for Linux.  

## Status
This is a tool I tried to make back in around 2018, but never finished because of my lack of skills and interest in GUI development. I'm now using this as an excuse to learn modern vibe coding, accelerated and improved by my own experience doing my initial attempt.  
This is very much a WIP, but in my opinion it's already better to use compared to the traditional method of writing assembler, objdump, format it, insert it in a C program and running it via GDB.  

## Features
- [x] Linux support
- [x] x86_64 assembler
- [x] Native arch debugger (x86_64)
- [x] Cross-architecture assembler (x86, x86_64, ARM, ARM-Thumb, AArch64, RiscV)
- [x] Intel syntax
- [x] AT&T syntax
- [x] Linux x86_64 syscall reference
- [x] IP address structure packer
- [x] Registers viewer/editor
- [x] Memory viewer/editor
- [x] Bad characters detection and highlight
- [x] Multi-langueage shellcode formatting (raw hex, python, C, Rust)
- [ ] In-browser support using wasm
- [ ] Cross-platform support (Windows, macOS)
- [ ] Cross-architecture debugging support (via emulation)
- [ ] Native architecture debugger for non x86_64 architectures
- [ ] Payload encoding (e.g. shellcode compression, encryption, badchars avoidance)
- [ ] Payload encoding plugin system (using wasm plugins)
- [ ] Instruction set reference

## Run
To run the project, clone the repository and run:
```bash
cargo run
```

## Demo
![Shellcide GUI](./assets/screenshot1.png)  

## License
This project is dual-licensed under the MIT and Apache 2.0 licenses.

## AI disclaimer
Code is mostly AI generated.
