use keystone_engine::{Keystone, Arch, Mode, OptionType, OptionValue};

fn strip_comments(code: &str) -> String {
    let mut clean_lines = Vec::new();
    for line in code.lines() {
        let mut clean_line = String::new();
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escaped = false;
        for c in line.chars() {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '\'' && !in_double_quote {
                in_single_quote = !in_single_quote;
            } else if c == '"' && !in_single_quote {
                in_double_quote = !in_double_quote;
            } else if c == ';' && !in_single_quote && !in_double_quote {
                break;
            }
            clean_line.push(c);
        }
        clean_lines.push(clean_line);
    }
    clean_lines.join("\n")
}

/// Assembles assembly code into x86_64 machine code bytes.
///
/// # Arguments
/// * `code` - The assembly source code string.
/// * `base_address` - The base address where the code will be loaded in memory.
/// * `att_syntax` - If true, uses AT&T syntax. Otherwise, uses Intel syntax.
pub fn assemble(code: &str, base_address: u64, att_syntax: bool) -> Result<Vec<u8>, String> {
    // Strip comments to prevent Keystone compilation errors
    let clean_code = strip_comments(code);

    // Initialize Keystone engine for x86_64
    let engine = Keystone::new(Arch::X86, Mode::MODE_64)
        .map_err(|e| format!("Failed to initialize Keystone engine: {:?}", e))?;

    if att_syntax {
        engine
            .option(OptionType::SYNTAX, OptionValue::SYNTAX_ATT)
            .map_err(|e| format!("Failed to set syntax option: {:?}", e))?;
    }

    // Perform assembly compilation
    let result = engine
        .asm(clean_code, base_address)
        .map_err(|e| format!("Assembly failed: {:?}", e))?;

    Ok(result.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler_intel() {
        let code = "mov rax, 10";
        let res = assemble(code, 0x10000000, false).unwrap();
        assert_eq!(res, vec![0x48, 0xC7, 0xC0, 0x0A, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_assembler_att() {
        let code = "movq $10, %rax";
        let res = assemble(code, 0x10000000, true).unwrap();
        assert_eq!(res, vec![0x48, 0xC7, 0xC0, 0x0A, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_assembler_with_comments() {
        let code = "mov rax, 10 ; load 10 into rax\n; whole line comment\nmov rbx, 20";
        let res = assemble(code, 0x10000000, false).unwrap();
        assert!(!res.is_empty());
    }

    #[test]
    fn test_semicolon_in_quotes() {
        let clean = strip_comments("mov al, ';'\nmov bl, \";\"");
        assert_eq!(clean, "mov al, ';'\nmov bl, \";\"");
    }

    #[test]
    fn test_escaped_quote_in_quotes() {
        let clean = strip_comments("mov al, '\\'' ; comment");
        assert_eq!(clean, "mov al, '\\'' ");
    }
}

