use capstone::prelude::*;
use crate::app::TargetArch;

#[derive(Debug, Clone)]
pub struct DisassembledInstruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub op_str: String,
}

/// Disassembles machine code bytes starting from a base address.
///
/// # Arguments
/// * `code` - Raw binary machine code.
/// * `base_address` - The base instruction pointer address.
/// * `att_syntax` - If true, uses AT&T syntax. If false, uses Intel syntax.
pub fn disassemble_code(code: &[u8], base_address: u64, att_syntax: bool, arch: TargetArch) -> Vec<DisassembledInstruction> {
    if code.is_empty() {
        return Vec::new();
    }

    let cs = match arch {
        TargetArch::X86_64 => {
            let mut builder = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode64);
            if att_syntax {
                builder = builder.syntax(arch::x86::ArchSyntax::Att);
            } else {
                builder = builder.syntax(arch::x86::ArchSyntax::Intel);
            }
            builder.build()
        }
        TargetArch::X86 => {
            let mut builder = Capstone::new()
                .x86()
                .mode(arch::x86::ArchMode::Mode32);
            if att_syntax {
                builder = builder.syntax(arch::x86::ArchSyntax::Att);
            } else {
                builder = builder.syntax(arch::x86::ArchSyntax::Intel);
            }
            builder.build()
        }
        TargetArch::Arm => Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Arm)
            .build(),
        TargetArch::Thumb => Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Thumb)
            .build(),
        TargetArch::Aarch64 => Capstone::new()
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .build(),
        TargetArch::Riscv => Capstone::new()
            .riscv()
            .mode(arch::riscv::ArchMode::RiscV64)
            .build(),
    };

    let cs = match cs {
        Ok(cs) => cs,
        Err(_) => return Vec::new(),
    };

    let insns = match cs.disasm_all(code, base_address) {
        Ok(insns) => insns,
        Err(_) => return Vec::new(),
    };

    let mut instructions = Vec::new();
    for insn in insns.as_ref() {
        let mnemonic = insn.mnemonic().unwrap_or("").to_string();
        let op_str = insn.op_str().unwrap_or("").to_string();
        instructions.push(DisassembledInstruction {
            address: insn.address(),
            bytes: insn.bytes().to_vec(),
            mnemonic,
            op_str,
        });
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassembler_intel() {
        let bytes = vec![0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00];
        let insts = disassemble_code(&bytes, 0x10000000, false, TargetArch::X86_64);
        assert_eq!(insts.len(), 1);
        assert_eq!(insts[0].mnemonic, "mov");
        assert_eq!(insts[0].op_str, "rax, 1");
    }

    #[test]
    fn test_disassembler_att() {
        let bytes = vec![0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00];
        let insts = disassemble_code(&bytes, 0x10000000, true, TargetArch::X86_64);
        assert_eq!(insts.len(), 1);
        assert_eq!(insts[0].mnemonic, "movq");
        assert_eq!(insts[0].op_str, "$1, %rax");
    }
}

