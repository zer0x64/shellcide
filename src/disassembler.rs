use iced_x86::{Decoder, DecoderOptions, Formatter, GasFormatter, IntelFormatter, Instruction};

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
/// * `att_syntax` - If true, uses AT&T syntax (GasFormatter). If false, uses Intel syntax.
pub fn disassemble_code(code: &[u8], base_address: u64, att_syntax: bool) -> Vec<DisassembledInstruction> {
    if code.is_empty() {
        return Vec::new();
    }

    let mut decoder = Decoder::new(64, code, DecoderOptions::NONE);
    decoder.set_ip(base_address);

    let mut instructions = Vec::new();
    let mut instruction = Instruction::default();

    // Create formatter conditionally
    let mut intel_formatter = if !att_syntax { Some(IntelFormatter::new()) } else { None };
    let mut gas_formatter = if att_syntax { Some(GasFormatter::new()) } else { None };

    while decoder.can_decode() {
        let offset = decoder.position();
        decoder.decode_out(&mut instruction);
        let len = instruction.len();

        let inst_bytes = if offset + len <= code.len() {
            code[offset..(offset + len)].to_vec()
        } else {
            code[offset..].to_vec()
        };

        let mut formatted = String::new();
        if att_syntax {
            gas_formatter.as_mut().unwrap().format(&instruction, &mut formatted);
        } else {
            intel_formatter.as_mut().unwrap().format(&instruction, &mut formatted);
        }

        // Format to split mnemonic and operands
        let parts: Vec<&str> = formatted.splitn(2, ' ').collect();
        let mnemonic = parts[0].to_string();
        let op_str = if parts.len() > 1 {
            parts[1].to_string()
        } else {
            String::new()
        };

        instructions.push(DisassembledInstruction {
            address: instruction.ip(),
            bytes: inst_bytes,
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
        let insts = disassemble_code(&bytes, 0x10000000, false);
        assert_eq!(insts.len(), 1);
        assert_eq!(insts[0].mnemonic, "mov");
        assert_eq!(insts[0].op_str, "rax,1");
    }

    #[test]
    fn test_disassembler_att() {
        let bytes = vec![0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00];
        let insts = disassemble_code(&bytes, 0x10000000, true);
        assert_eq!(insts.len(), 1);
        assert_eq!(insts[0].mnemonic, "mov");
        assert_eq!(insts[0].op_str, "$1,%rax");
    }
}

