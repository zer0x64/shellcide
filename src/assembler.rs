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

/// Parses bad character strings into a set of bytes.
/// Supports space/comma/newline separation, hex prefixes/escapes, hex pairs, ranges, and decimal fallback.
pub fn parse_bad_characters(input: &str) -> std::collections::HashSet<u8> {
    let mut bad_chars = std::collections::HashSet::new();
    
    // Split input by whitespace, commas, semicolons, or newlines
    let tokens = input.split(|c: char| c == ' ' || c == ',' || c == ';' || c == '\n' || c == '\r' || c == '\t');
    
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
    let (cleaned, is_explicit_hex) = if lowercase_s.starts_with("\\x") {
        (&s[2..], true)
    } else if lowercase_s.starts_with("0x") {
        (&s[2..], true)
    } else {
        (s, false)
    };
    
    let has_hex_chars = cleaned.chars().any(|c| {
        let lc = c.to_ascii_lowercase();
        c.is_ascii_alphabetic() && lc >= 'a' && lc <= 'f'
    });
    let is_hex_pair = cleaned.len() == 2;
    
    if is_explicit_hex || has_hex_chars || is_hex_pair {
        // Try parsing as hex
        if let Ok(b) = u8::from_str_radix(cleaned, 16) {
            return Some(b);
        }
    }
    
    // Fallback: try parsing as decimal, then try as hex if decimal fails
    if let Ok(b) = cleaned.parse::<u8>() {
        Some(b)
    } else if let Ok(b) = u8::from_str_radix(cleaned, 16) {
        Some(b)
    } else {
        None
    }
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
        let code = "movq $0xa, %rax";
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
}

