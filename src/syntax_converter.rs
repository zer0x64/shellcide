fn is_register(s: &str) -> bool {
    let s = s.trim().to_lowercase();
    let s = s.strip_prefix('%').unwrap_or(&s);
    if register_size(s).is_some() {
        return true;
    }
    match s {
        "rip" | "eip" | "ip" | "cs" | "ds" | "ss" | "es" | "fs" | "gs" => true,
        _ if s.starts_with("xmm") || s.starts_with("ymm") || s.starts_with("zmm") => s
            .get(3..)
            .and_then(|num| num.parse::<u32>().ok())
            .is_some_and(|n| n <= 31),
        _ if s.starts_with("cr") || s.starts_with("dr") => s
            .get(2..)
            .and_then(|num| num.parse::<u32>().ok())
            .is_some_and(|n| n <= 15),
        _ => false,
    }
}

fn register_size(reg: &str) -> Option<char> {
    let r = reg.trim().to_lowercase();
    let r = r.strip_prefix('%').unwrap_or(&r);
    match r {
        "rax" | "rbx" | "rcx" | "rdx" | "rsi" | "rdi" | "rbp" | "rsp" | "r8" | "r9" | "r10"
        | "r11" | "r12" | "r13" | "r14" | "r15" => Some('q'),
        "eax" | "ebx" | "ecx" | "edx" | "esi" | "edi" | "ebp" | "esp" | "r8d" | "r9d" | "r10d"
        | "r11d" | "r12d" | "r13d" | "r14d" | "r15d" => Some('l'),
        "ax" | "bx" | "cx" | "dx" | "si" | "di" | "bp" | "sp" | "r8w" | "r9w" | "r10w" | "r11w"
        | "r12w" | "r13w" | "r14w" | "r15w" => Some('w'),
        "al" | "bl" | "cl" | "dl" | "sil" | "dil" | "bpl" | "spl" | "r8b" | "r9b" | "r10b"
        | "r11b" | "r12b" | "r13b" | "r14b" | "r15b" | "ah" | "bh" | "ch" | "dh" => Some('b'),
        _ => None,
    }
}

fn is_immediate_val(s: &str) -> bool {
    let s = s.trim();
    let rest = s
        .strip_prefix('+')
        .or_else(|| s.strip_prefix('-'))
        .unwrap_or(s);
    if rest.is_empty() {
        return false;
    }
    if let Some(hex_part) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
        !hex_part.is_empty() && hex_part.chars().all(|c| c.is_ascii_hexdigit())
    } else {
        rest.chars().all(|c| c.is_ascii_digit())
    }
}

fn is_quoted(s: &str) -> bool {
    let s = s.trim();
    (s.starts_with('\'') && s.ends_with('\'')) || (s.starts_with('"') && s.ends_with('"'))
}

fn is_standard_mnemonic(m: &str) -> bool {
    let m_lower = m.to_lowercase();
    matches!(
        m_lower.as_str(),
        "mov"
            | "xor"
            | "add"
            | "sub"
            | "cmp"
            | "or"
            | "and"
            | "shl"
            | "shr"
            | "sar"
            | "test"
            | "inc"
            | "dec"
            | "push"
            | "pop"
            | "movzx"
            | "movsx"
            | "xchg"
            | "lea"
    )
}

fn is_directive(m: &str) -> bool {
    let m_lower = m.to_lowercase();
    m_lower.starts_with('.') || matches!(m_lower.as_str(), "db" | "dw" | "dd" | "dq")
}

fn is_jump_or_call(m: &str) -> bool {
    let m_lower = m.to_lowercase();
    matches!(
        m_lower.as_str(),
        "jmp"
            | "call"
            | "je"
            | "jne"
            | "jg"
            | "jl"
            | "ja"
            | "jb"
            | "jae"
            | "jbe"
            | "jge"
            | "jle"
            | "js"
            | "jns"
            | "jo"
            | "jno"
            | "jc"
            | "jnc"
            | "jp"
            | "jnp"
            | "jcxz"
            | "jecxz"
            | "rcxz"
    )
}

fn find_char_outside_quotes<F>(s: &str, predicate: F) -> Option<usize>
where
    F: Fn(char) -> bool,
{
    let mut in_single = false;
    let mut in_double = false;
    let mut escaped = false;

    for (i, c) in s.char_indices() {
        if escaped {
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '\'' && !in_double {
            in_single = !in_single;
        } else if c == '"' && !in_single {
            in_double = !in_double;
        } else if !in_single && !in_double && predicate(c) {
            return Some(i);
        }
    }
    None
}

fn parse_line(line: &str) -> (Option<String>, Option<String>, Option<String>) {
    let comment_start = find_char_outside_quotes(line, |c| c == ';' || c == '#');
    let (code_part, comment_part) = if let Some(idx) = comment_start {
        let (code, comment) = line.split_at(idx);
        (code, Some(comment.to_string()))
    } else {
        (line, None)
    };

    let label_end = find_char_outside_quotes(code_part, |c| c == ':');
    let (label_part, inst_part) = if let Some(idx) = label_end {
        let (label, rest) = code_part.split_at(idx + 1);
        (Some(label.trim().to_string()), rest)
    } else {
        (None, code_part)
    };

    let inst = {
        let trimmed = inst_part.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    };

    (label_part, inst, comment_part)
}

fn split_operands(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut operands = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut bracket_depth: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut escaped = false;

    for c in s.chars() {
        if escaped {
            escaped = false;
        } else {
            match c {
                '\\' => escaped = true,
                '\'' if !in_double => in_single = !in_single,
                '"' if !in_single => in_double = !in_double,
                '[' if !in_single && !in_double => bracket_depth += 1,
                ']' if !in_single && !in_double => bracket_depth = bracket_depth.saturating_sub(1),
                '(' if !in_single && !in_double => paren_depth += 1,
                ')' if !in_single && !in_double => paren_depth = paren_depth.saturating_sub(1),
                ',' if bracket_depth == 0 && paren_depth == 0 && !in_single && !in_double => {
                    operands.push(current.trim().to_string());
                    current.clear();
                    continue;
                }
                _ => {}
            }
        }
        current.push(c);
    }
    let trimmed = current.trim();
    if !trimmed.is_empty() || s.trim().ends_with(',') {
        operands.push(trimmed.to_string());
    }
    operands
}

fn split_terms(expr: &str) -> Vec<(char, String)> {
    let mut terms = Vec::new();
    let mut current_term = String::new();
    let mut current_sign = '+';

    for c in expr.chars() {
        if c == '+' || c == '-' {
            let trimmed = current_term.trim();
            if !trimmed.is_empty() {
                terms.push((current_sign, trimmed.to_string()));
                current_term.clear();
            }
            current_sign = c;
        } else {
            current_term.push(c);
        }
    }
    let trimmed = current_term.trim();
    if !trimmed.is_empty() {
        terms.push((current_sign, trimmed.to_string()));
    }
    terms
}

fn strip_size_prefix(operand: &str) -> (String, Option<char>) {
    let trimmed = operand.trim();
    let lower = trimmed.to_lowercase();
    let (prefix_len, suffix) = if lower.starts_with("byte ptr ") {
        (9, 'b')
    } else if lower.starts_with("word ptr ") {
        (9, 'w')
    } else if lower.starts_with("dword ptr ") {
        (10, 'l')
    } else if lower.starts_with("qword ptr ") {
        (10, 'q')
    } else {
        return (trimmed.to_string(), None);
    };
    (trimmed[prefix_len..].trim().to_string(), Some(suffix))
}

fn strip_att_mnemonic_suffix(m: &str) -> (String, Option<char>) {
    if m.len() > 1 {
        if let Some(prefix) = m.get(..m.len() - 1) {
            if let Some(last_char) = m.chars().last() {
                if matches!(last_char, 'b' | 'w' | 'l' | 'q') && is_standard_mnemonic(prefix) {
                    return (prefix.to_string(), Some(last_char));
                }
            }
        }
    }
    (m.to_string(), None)
}

fn format_line(
    label: Option<&str>,
    reconstructed_inst: Option<&str>,
    comment: Option<&str>,
) -> String {
    let mut line_result = String::new();
    if let Some(lbl) = label {
        line_result.push_str(lbl);
    }
    if let Some(inst) = reconstructed_inst {
        if label.is_some() {
            line_result.push(' ');
        } else {
            line_result.push_str("    ");
        }
        line_result.push_str(inst);
    }
    if let Some(cmt) = comment {
        if label.is_some() || reconstructed_inst.is_some() {
            line_result.push_str("  ");
        }
        line_result.push_str(cmt);
    }
    line_result
}

pub fn intel_to_att(code: &str) -> String {
    let mut lines = Vec::new();
    for line in code.lines() {
        let (label, inst, comment) = parse_line(line);
        let reconstructed_inst = inst.as_ref().map(|inst_str| {
            let trimmed = inst_str.trim();
            let (mnemonic, operands_str) = trimmed
                .split_once(char::is_whitespace)
                .map(|(m, ops)| (m.to_string(), ops.trim().to_string()))
                .unwrap_or_else(|| (trimmed.to_string(), String::new()));
            let operands = split_operands(&operands_str);

            if is_directive(&mnemonic) {
                if operands.is_empty() {
                    mnemonic
                } else {
                    format!("{} {}", mnemonic, operands.join(", "))
                }
            } else {
                let mut size_suffix = None;
                let mut translated_operands = Vec::new();

                for op in operands {
                    let (op_stripped, suffix) = strip_size_prefix(&op);
                    if suffix.is_some() {
                        size_suffix = suffix;
                    } else if size_suffix.is_none() && is_register(&op_stripped) {
                        size_suffix = register_size(&op_stripped);
                    }

                    let translated = if is_register(&op_stripped) {
                        format!("%{}", op_stripped)
                    } else if is_immediate_val(&op_stripped) || is_quoted(&op_stripped) {
                        format!("${}", op_stripped)
                    } else if let Some(expr) = op_stripped
                        .strip_prefix('[')
                        .and_then(|o| o.strip_suffix(']'))
                    {
                        let terms = split_terms(expr);

                        let mut base: Option<String> = None;
                        let mut index: Option<String> = None;
                        let mut scale: Option<String> = None;
                        let mut disp: Option<String> = None;

                        for (sign, term) in terms {
                            if term.contains('*') {
                                let parts: Vec<&str> = term.split('*').collect();
                                if parts.len() == 2 {
                                    let left = parts[0].trim().to_string();
                                    let right = parts[1].trim().to_string();
                                    if is_register(&left) {
                                        index = Some(left);
                                        scale = Some(right);
                                    } else if is_register(&right) {
                                        index = Some(right);
                                        scale = Some(left);
                                    } else {
                                        index = Some(left);
                                        scale = Some(right);
                                    }
                                }
                            } else if is_register(&term) {
                                if base.is_none() {
                                    base = Some(term);
                                } else if index.is_none() {
                                    index = Some(term);
                                    scale = Some("1".to_string());
                                }
                            } else {
                                let signed_term = if sign == '-' {
                                    format!("-{}", term)
                                } else {
                                    term
                                };
                                disp = Some(signed_term);
                            }
                        }

                        // Format AT&T memory operand
                        let mut inner = String::new();
                        if let Some(b) = &base {
                            inner.push_str(&format!("%{}", b));
                        }
                        if let Some(i) = &index {
                            if !inner.is_empty() {
                                inner.push_str(", ");
                            } else {
                                inner.push_str(", "); // leading comma if base is None
                            }
                            inner.push_str(&format!("%{}", i));
                            if let Some(s) = &scale {
                                if s != "1" {
                                    inner.push_str(&format!(", {}", s));
                                }
                            }
                        }
                        let disp_str = disp.unwrap_or_default();
                        if inner.is_empty() {
                            disp_str
                        } else {
                            format!("{}({})", disp_str, inner)
                        }
                    } else {
                        op_stripped
                    };
                    translated_operands.push(translated);
                }

                // Swap operands if > 1
                if translated_operands.len() > 1 {
                    translated_operands.reverse();
                }

                // Append size suffix to mnemonic if standard
                let mut final_mnemonic = mnemonic;
                if let Some(s) = size_suffix {
                    if is_standard_mnemonic(&final_mnemonic) {
                        final_mnemonic.push(s);
                    }
                }

                if translated_operands.is_empty() {
                    final_mnemonic
                } else {
                    format!("{} {}", final_mnemonic, translated_operands.join(", "))
                }
            }
        });

        lines.push(format_line(
            label.as_deref(),
            reconstructed_inst.as_deref(),
            comment.as_deref(),
        ));
    }
    lines.join("\n")
}

pub fn att_to_intel(code: &str) -> String {
    let mut lines = Vec::new();
    for line in code.lines() {
        let (label, inst, comment) = parse_line(line);
        let reconstructed_inst = inst.as_ref().map(|inst_str| {
            let trimmed = inst_str.trim();
            let (mnemonic, operands_str) = trimmed
                .split_once(char::is_whitespace)
                .map(|(m, ops)| (m.to_string(), ops.trim().to_string()))
                .unwrap_or_else(|| (trimmed.to_string(), String::new()));
            let operands = split_operands(&operands_str);

            if is_directive(&mnemonic) {
                if operands.is_empty() {
                    mnemonic
                } else {
                    format!("{} {}", mnemonic, operands.join(", "))
                }
            } else {
                let (mnemonic_no_suffix, size_suffix) = strip_att_mnemonic_suffix(&mnemonic);
                let size_prefix = match size_suffix {
                    Some('b') => Some("byte ptr "),
                    Some('w') => Some("word ptr "),
                    Some('l') => Some("dword ptr "),
                    Some('q') => Some("qword ptr "),
                    _ => None,
                };

                let mut translated_operands = Vec::new();
                for op in operands {
                    let translated = if let Some(reg) = op.strip_prefix('%') {
                        if is_register(reg) {
                            reg.to_string()
                        } else {
                            op
                        }
                    } else if let Some(imm) = op.strip_prefix('$') {
                        imm.to_string()
                    } else {
                        let is_mem = op.contains('(') || !is_jump_or_call(&mnemonic_no_suffix);

                        if is_mem {
                            let mut base: Option<String> = None;
                            let mut index: Option<String> = None;
                            let mut scale: Option<String> = None;
                            let mut disp: Option<String> = None;

                            if let Some(open_paren_idx) = op.find('(') {
                                let disp_part = op[..open_paren_idx].trim();
                                if !disp_part.is_empty() {
                                    disp = Some(disp_part.to_string());
                                }

                                if let Some(close_paren_idx) = op.find(')') {
                                    let inner = &op[open_paren_idx + 1..close_paren_idx];
                                    let parts: Vec<&str> =
                                        inner.split(',').map(|s| s.trim()).collect();

                                    if let Some(&b) = parts.first() {
                                        if !b.is_empty() {
                                            base =
                                                Some(b.strip_prefix('%').unwrap_or(b).to_string());
                                        }
                                    }
                                    if let Some(&idx) = parts.get(1) {
                                        if !idx.is_empty() {
                                            index = Some(
                                                idx.strip_prefix('%').unwrap_or(idx).to_string(),
                                            );
                                        }
                                    }
                                    if let Some(&sc) = parts.get(2) {
                                        if !sc.is_empty() {
                                            scale = Some(sc.to_string());
                                        }
                                    }
                                }
                            } else {
                                disp = Some(op);
                            }

                            let mut reg_terms = Vec::new();
                            if let Some(b) = base {
                                reg_terms.push(b);
                            }
                            if let Some(idx) = index {
                                if let Some(sc) = scale {
                                    if sc != "1" {
                                        reg_terms.push(format!("{}*{}", idx, sc));
                                    } else {
                                        reg_terms.push(idx);
                                    }
                                } else {
                                    reg_terms.push(idx);
                                }
                            }

                            let reg_str = reg_terms.join(" + ");
                            let mut mem_expr = String::new();
                            if reg_str.is_empty() {
                                if let Some(d) = disp {
                                    mem_expr = d;
                                }
                            } else {
                                if let Some(d) = disp {
                                    if let Some(stripped) = d.strip_prefix('-') {
                                        mem_expr = format!("{} - {}", reg_str, stripped.trim());
                                    } else {
                                        mem_expr = format!("{} + {}", reg_str, d);
                                    }
                                } else {
                                    mem_expr = reg_str;
                                }
                            }

                            let mut formatted_mem = format!("[{}]", mem_expr);
                            if let Some(prefix) = size_prefix {
                                formatted_mem = format!("{}{}", prefix, formatted_mem);
                            }
                            formatted_mem
                        } else {
                            op
                        }
                    };
                    translated_operands.push(translated);
                }

                // Swap operands if > 1
                if translated_operands.len() > 1 {
                    translated_operands.reverse();
                }

                if translated_operands.is_empty() {
                    mnemonic_no_suffix
                } else {
                    format!("{} {}", mnemonic_no_suffix, translated_operands.join(", "))
                }
            }
        });

        lines.push(format_line(
            label.as_deref(),
            reconstructed_inst.as_deref(),
            comment.as_deref(),
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_to_att() {
        assert_eq!(intel_to_att("mov rax, rbx"), "    movq %rbx, %rax");
        assert_eq!(intel_to_att("mov rax, 10"), "    movq $10, %rax");
        assert_eq!(intel_to_att("mov rax, 0x10"), "    movq $0x10, %rax");
        assert_eq!(intel_to_att("sub rsp, rcx"), "    subq %rcx, %rsp");
        assert_eq!(
            intel_to_att("mov byte ptr [rsi], al"),
            "    movb %al, (%rsi)"
        );
        assert_eq!(
            intel_to_att("mov qword ptr [rsi + rdi*4 + 8], rax"),
            "    movq %rax, 8(%rsi, %rdi, 4)"
        );
        assert_eq!(
            intel_to_att("mov dword ptr [rsi + rdi*4 - 8], eax"),
            "    movl %eax, -8(%rsi, %rdi, 4)"
        );
        assert_eq!(
            intel_to_att("mov dword ptr [rdi*4 - 8], eax"),
            "    movl %eax, -8(, %rdi, 4)"
        );
        assert_eq!(intel_to_att("jmp _start"), "    jmp _start");
        assert_eq!(intel_to_att(".byte 0x10, 0x20"), "    .byte 0x10, 0x20");
    }

    #[test]
    fn test_att_to_intel() {
        assert_eq!(att_to_intel("mov %rbx, %rax"), "    mov rax, rbx");
        assert_eq!(att_to_intel("mov $10, %rax"), "    mov rax, 10");
        assert_eq!(att_to_intel("mov $0x10, %rax"), "    mov rax, 0x10");
        assert_eq!(
            att_to_intel("movb %al, (%rsi)"),
            "    mov byte ptr [rsi], al"
        );
        assert_eq!(
            att_to_intel("movq %rax, 8(%rsi, %rdi, 4)"),
            "    mov qword ptr [rsi + rdi*4 + 8], rax"
        );
        assert_eq!(
            att_to_intel("movl %eax, -8(%rsi, %rdi, 4)"),
            "    mov dword ptr [rsi + rdi*4 - 8], eax"
        );
        assert_eq!(
            att_to_intel("movl %eax, -8(, %rdi, 4)"),
            "    mov dword ptr [rdi*4 - 8], eax"
        );
        assert_eq!(att_to_intel("jmp _start"), "    jmp _start");
        assert_eq!(att_to_intel(".byte 0x10, 0x20"), "    .byte 0x10, 0x20");
    }

    #[test]
    fn test_binary_equivalence() {
        let intel_code = "
            mov rsi, 0x20000000
            mov dword ptr [rsi], 0x6c6c6548
            mov dword ptr [rsi+4], 0x73202c6f
            mov rax, 1
            mov rdi, 1
            mov rdx, 18
            syscall
        ";
        let att_code = intel_to_att(intel_code);
        let intel_bytes = crate::assembler::assemble(
            intel_code,
            0x10000000,
            false,
            crate::app::TargetArch::X86_64,
        )
        .unwrap();
        let att_bytes =
            crate::assembler::assemble(&att_code, 0x10000000, true, crate::app::TargetArch::X86_64)
                .unwrap();
        assert_eq!(intel_bytes, att_bytes);
    }
}
