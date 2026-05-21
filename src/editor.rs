use eframe::egui;

/// Evaluates if a word is an x86_64 instruction mnemonic.
fn is_mnemonic(word: &str) -> bool {
    matches!(
        word,
        "mov" | "add" | "sub" | "mul" | "imul" | "div" | "idiv" | "cmp" | "jmp" | 
        "je" | "jne" | "jg" | "jge" | "jl" | "jle" | "jz" | "jnz" | "js" | "jns" | 
        "jc" | "jnc" | "call" | "ret" | "push" | "pop" | "syscall" | "int" | "nop" | 
        "leave" | "enter" | "inc" | "dec" | "xor" | "or" | "and" | "shl" | "shr" | 
        "lea" | "db" | "dw" | "dd" | "dq"
    )
}

/// Evaluates if a word is an x86_64 register.
fn is_register(word: &str) -> bool {
    matches!(
        word,
        // 64-bit general purpose & special registers
        "rax" | "rbx" | "rcx" | "rdx" | "rsi" | "rdi" | "rbp" | "rsp" | 
        "r8" | "r9" | "r10" | "r11" | "r12" | "r13" | "r14" | "r15" | 
        "rip" | "rflags" |
        // 32-bit general purpose
        "eax" | "ebx" | "ecx" | "edx" | "esi" | "edi" | "ebp" | "esp" | 
        "r8d" | "r9d" | "r10d" | "r11d" | "r12d" | "r13d" | "r14d" | "r15d" |
        // 16-bit
        "ax" | "bx" | "cx" | "dx" | "si" | "di" | "bp" | "sp" | 
        "r8w" | "r9w" | "r10w" | "r11w" | "r12w" | "r13w" | "r14w" | "r15w" |
        // 8-bit
        "al" | "ah" | "bl" | "bh" | "cl" | "ch" | "dl" | "dh" | 
        "sil" | "dil" | "bpl" | "spl" | "r8b" | "r9b" | "r10b" | "r11b" | "r12b" | 
        "r13b" | "r14b" | "r15b"
    )
}

/// Evaluates if a word represents a number (decimal, hex, binary).
fn is_number(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    
    // Check if standard decimal or hex
    let first = word.chars().next().unwrap();
    if first.is_ascii_digit() {
        return true;
    }
    
    // Check if negative number
    if first == '-' && word.len() > 1 && word.chars().nth(1).unwrap().is_ascii_digit() {
        return true;
    }
    
    // Check hex constants (like 0xABC or A0h)
    if word.starts_with("0x") || word.starts_with("0b") {
        return true;
    }
    
    if word.ends_with('h') && word.len() > 1 && word.chars().next().unwrap().is_ascii_alphanumeric() {
        return true;
    }

    false
}

struct SyntaxColors {
    default: egui::Color32,
    mnemonic: egui::Color32,
    register: egui::Color32,
    number: egui::Color32,
    directive: egui::Color32,
}

/// Flushes the active token string and appends formatted text to the LayoutJob.
fn flush_token(
    token: &mut String,
    job: &mut egui::text::LayoutJob,
    font_id: &egui::FontId,
    colors: &SyntaxColors,
) {
    if token.is_empty() {
        return;
    }

    let token_lower = token.to_lowercase();

    let color = if token.starts_with('.') {
        colors.directive
    } else if is_mnemonic(&token_lower) {
        colors.mnemonic
    } else if is_register(&token_lower) {
        colors.register
    } else if is_number(&token_lower) {
        colors.number
    } else {
        colors.default
    };

    job.append(
        token,
        0.0,
        egui::TextFormat {
            font_id: font_id.clone(),
            color,
            ..Default::default()
        },
    );
    token.clear();
}

/// Highlight assembly code and return an egui::text::LayoutJob.
pub fn highlight_assembly(_ui: &egui::Ui, code: &str) -> egui::text::LayoutJob {
    let font_id = egui::FontId::monospace(14.0);
    let mut job = egui::text::LayoutJob::default();

    // Cyber Cyan Color Palette
    let colors = SyntaxColors {
        default: egui::Color32::from_rgb(223, 230, 233),   // Soft white/gray
        mnemonic: egui::Color32::from_rgb(0, 206, 203),    // Cyber Cyan
        register: egui::Color32::from_rgb(253, 121, 168),  // Neon Pink
        number: egui::Color32::from_rgb(250, 177, 160),    // Soft Orange
        directive: egui::Color32::from_rgb(116, 185, 255),  // Ice Blue
    };
    let color_comment = egui::Color32::from_rgb(99, 110, 114);    // Slate Gray (dimmed)
    let color_label = egui::Color32::from_rgb(85, 239, 196);      // Toxic Green

    let mut chars = code.chars().peekable();
    let mut current_token = String::new();

    while let Some(c) = chars.next() {
        if c == ';' || c == '#' {
            // Flush any existing buffered token
            flush_token(
                &mut current_token,
                &mut job,
                &font_id,
                &colors,
            );

            // Accumulate rest of line as a comment
            let mut comment = String::new();
            comment.push(c);
            while let Some(&next_c) = chars.peek() {
                if next_c == '\n' {
                    break;
                }
                comment.push(chars.next().unwrap());
            }

            job.append(
                &comment,
                0.0,
                egui::TextFormat {
                    font_id: font_id.clone(),
                    color: color_comment,
                    ..Default::default()
                },
            );
        } else if c.is_alphanumeric() || c == '_' || c == '.' || c == '@' {
            current_token.push(c);
        } else {
            // Separator hit. Flush previous token first.
            if c == ':' && !current_token.is_empty() {
                current_token.push(c);
                job.append(
                    &current_token,
                    0.0,
                    egui::TextFormat {
                        font_id: font_id.clone(),
                        color: color_label,
                        ..Default::default()
                    },
                );
                current_token.clear();
            } else {
                flush_token(
                    &mut current_token,
                    &mut job,
                    &font_id,
                    &colors,
                );

                let mut sep = String::new();
                sep.push(c);
                job.append(
                    &sep,
                    0.0,
                    egui::TextFormat {
                        font_id: font_id.clone(),
                        color: colors.default,
                        ..Default::default()
                    },
                );
            }
        }
    }

    // Flush any remaining text
    flush_token(
        &mut current_token,
        &mut job,
        &font_id,
        &colors,
    );

    job
}
