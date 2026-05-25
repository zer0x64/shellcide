use iced_x86::{Decoder, DecoderOptions, Formatter, IntelFormatter, GasFormatter};
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

    let bitness = match arch {
        TargetArch::X86_64 => 64,
        TargetArch::X86 => 32,
        _ => return Vec::new(),
    };

    let mut decoder = Decoder::with_ip(bitness, code, base_address, DecoderOptions::NONE);
    
    let mut formatter: Box<dyn Formatter> = if att_syntax {
        let mut f = GasFormatter::new();
        f.options_mut().set_space_after_operand_separator(true);
        f.options_mut().set_gas_show_mnemonic_size_suffix(true);
        Box::new(f)
    } else {
        let mut f = IntelFormatter::new();
        f.options_mut().set_space_after_operand_separator(true);
        Box::new(f)
    };

    let mut instructions = Vec::new();
    let mut offset = 0;

    for instruction in &mut decoder {
        let instr_len = instruction.len();
        let end = (offset + instr_len).min(code.len());
        let bytes = code[offset..end].to_vec();
        offset = end;

        let mut mnemonic = String::new();
        formatter.format_mnemonic(&instruction, &mut mnemonic);

        let mut op_str = String::new();
        formatter.format_all_operands(&instruction, &mut op_str);

        instructions.push(DisassembledInstruction {
            address: instruction.ip(),
            bytes,
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
