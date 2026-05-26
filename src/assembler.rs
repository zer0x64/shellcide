use crate::app::TargetArch;
use asm_rs::{Arch, Assembler, Syntax};

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
pub fn assemble(
    code: &str,
    base_address: u64,
    att_syntax: bool,
    arch: TargetArch,
) -> Result<Vec<u8>, String> {
    // Strip comments to prevent assembler parsing errors
    let clean_code = strip_comments(code);

    let asm_arch = match arch {
        TargetArch::X86_64 => Arch::X86_64,
        TargetArch::X86 => Arch::X86,
        TargetArch::Arm => Arch::Arm,
        TargetArch::Thumb => Arch::Thumb,
        TargetArch::Aarch64 => Arch::Aarch64,
        TargetArch::Riscv => Arch::Rv64,
    };

    // Initialize Assembler
    let mut assembler = Assembler::new(asm_arch);

    if att_syntax {
        assembler.syntax(Syntax::Att);
    } else {
        assembler.syntax(Syntax::Intel);
    }

    assembler.base_address(base_address);

    assembler
        .emit(&clean_code)
        .map_err(|e| format!("Assembly failed: {}", e))?;

    let result = assembler
        .finish()
        .map_err(|e| format!("Assembly compilation failed: {}", e))?;

    Ok(result.bytes().to_vec())
}

pub const DEMO_CODE: &str = r#"; Shellcide x86_64 Shellcode Demo
; Writes "Hello, shellcide!" to stdout and exits with code 42.
; Initial memory mapping:
; - Code:  0x10000000
; - Data:  0x20000000
; - Stack: 0x30000000

_start:
    ; 1. Write the string into the data section
    mov rsi, 0x20000000            ; Target data section address
    mov dword ptr [rsi], 0x6c6c6548     ; "Hell"
    mov dword ptr [rsi+4], 0x73202c6f   ; "o, s"
    mov dword ptr [rsi+8], 0x6c6c6568   ; "hell"
    mov dword ptr [rsi+12], 0x65646963  ; "cide"
    mov byte ptr [rsi+16], 0x21         ; "!"
    mov byte ptr [rsi+17], 0x0a         ; "\n"

    ; 2. Call sys_write (rax=1, rdi=1, rsi=buffer, rdx=18)
    mov rax, 1
    mov rdi, 1
    mov rdx, 18
    syscall

    ; 3. Call sys_exit (rax=60, rdi=42)
    mov rax, 60
    mov rdi, 42
    syscall
"#;

/// Parses bad character strings into a set of bytes.
/// Supports space/comma/newline separation, hex prefixes/escapes, hex pairs, ranges, and decimal fallback.
pub fn parse_bad_characters(input: &str) -> std::collections::HashSet<u8> {
    let mut bad_chars = std::collections::HashSet::new();

    // Split input by whitespace, commas, semicolons, or newlines
    let tokens = input.split(&[' ', ',', ';', '\n', '\r', '\t']);

    for token in tokens {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        // Handle ranges: e.g. "00-1f"
        if token.contains('-') {
            let parts: Vec<&str> = token.split('-').collect();
            if parts.len() == 2 {
                let start = parse_single_byte(parts[0].trim());
                let end = parse_single_byte(parts[1].trim());
                if let (Some(s), Some(e)) = (start, end) {
                    let min = s.min(e);
                    let max = s.max(e);
                    for b in min..=max {
                        bad_chars.insert(b);
                    }
                }
            }
        } else {
            if let Some(b) = parse_single_byte(token) {
                bad_chars.insert(b);
            }
        }
    }

    bad_chars
}

fn parse_single_byte(s: &str) -> Option<u8> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let lowercase_s = s.to_lowercase();

    // Check if it has a prefix
    let (cleaned, is_explicit_hex) =
        if lowercase_s.starts_with("\\x") || lowercase_s.starts_with("0x") {
            (&s[2..], true)
        } else {
            (s, false)
        };

    let has_hex_chars = cleaned.chars().any(|c| {
        let lc = c.to_ascii_lowercase();
        c.is_ascii_alphabetic() && ('a'..='f').contains(&lc)
    });
    let is_hex_pair = cleaned.len() == 2;

    if is_explicit_hex || has_hex_chars || is_hex_pair {
        // Try parsing as hex
        if let Ok(b) = u8::from_str_radix(cleaned, 16) {
            return Some(b);
        }
    }

    // Fallback: try parsing as decimal, then try as hex if decimal fails
    cleaned
        .parse::<u8>()
        .ok()
        .or_else(|| u8::from_str_radix(cleaned, 16).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler_intel() {
        let code = "mov rax, 10";
        let res = assemble(code, 0x10000000, false, TargetArch::X86_64).unwrap();
        assert_eq!(res, vec![0xB8, 0x0A, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_assembler_att() {
        let code = "movq $0xa, %rax";
        let res = assemble(code, 0x10000000, true, TargetArch::X86_64).unwrap();
        assert_eq!(res, vec![0xB8, 0x0A, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_assembler_with_comments() {
        let code = "mov rax, 10 ; load 10 into rax\n; whole line comment\nmov rbx, 20";
        let res = assemble(code, 0x10000000, false, TargetArch::X86_64).unwrap();
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

    #[test]
    fn test_parse_bad_characters() {
        let res = parse_bad_characters("\\x00 0A,0d;\\x90-95 255");
        assert!(res.contains(&0));
        assert!(res.contains(&10));
        assert!(res.contains(&13));
        assert!(res.contains(&144)); // 0x90
        assert!(res.contains(&145)); // 0x91
        assert!(res.contains(&149)); // 0x95
        assert!(res.contains(&255));
        assert!(!res.contains(&1));
        assert!(!res.contains(&143));
        assert!(!res.contains(&150));

        let ranges = parse_bad_characters("a-f");
        assert!(ranges.contains(&10));
        assert!(ranges.contains(&15));
        assert!(!ranges.contains(&9));
        assert!(!ranges.contains(&16));
    }

    #[test]
    fn test_demo_code_compilation() {
        let res = assemble(DEMO_CODE, 0x10000000, false, TargetArch::X86_64);
        assert!(res.is_ok(), "Compilation failed: {:?}", res.err());
    }
}
