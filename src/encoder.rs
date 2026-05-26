use crate::app::TargetArch;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionType {
    #[default]
    None,
    Rle,
}

impl CompressionType {
    pub const ALL: [Self; 2] = [Self::None, Self::Rle];
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Rle => "RLE (Run-Length Encoding)",
        }
    }
}

struct RegParts {
    r64: &'static str,
    r32: &'static str,
    r16: &'static str,
    r8: &'static str,
}

const REGISTERS: &[RegParts] = &[
    RegParts { r64: "rax", r32: "eax", r16: "ax", r8: "al" },
    RegParts { r64: "rcx", r32: "ecx", r16: "cx", r8: "cl" },
    RegParts { r64: "rdx", r32: "edx", r16: "dx", r8: "dl" },
    RegParts { r64: "rbx", r32: "ebx", r16: "bx", r8: "bl" },
    RegParts { r64: "rsi", r32: "esi", r16: "si", r8: "sil" },
    RegParts { r64: "rdi", r32: "edi", r16: "di", r8: "dil" },
    RegParts { r64: "rsp", r32: "esp", r16: "sp", r8: "spl" },
    RegParts { r64: "rbp", r32: "ebp", r16: "bp", r8: "bpl" },
    RegParts { r64: "r8", r32: "r8d", r16: "r8w", r8: "r8b" },
    RegParts { r64: "r9", r32: "r9d", r16: "r9w", r8: "r9b" },
    RegParts { r64: "r10", r32: "r10d", r16: "r10w", r8: "r10b" },
    RegParts { r64: "r11", r32: "r11d", r16: "r11w", r8: "r11b" },
    RegParts { r64: "r12", r32: "r12d", r16: "r12w", r8: "r12b" },
    RegParts { r64: "r13", r32: "r13d", r16: "r13w", r8: "r13b" },
    RegParts { r64: "r14", r32: "r14d", r16: "r14w", r8: "r14b" },
    RegParts { r64: "r15", r32: "r15d", r16: "r15w", r8: "r15b" },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncryptionType {
    #[default]
    None,
    Xor,
    Add,
}

impl EncryptionType {
    pub const ALL: [Self; 3] = [Self::None, Self::Xor, Self::Add];
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Xor => "XOR (Repeating Key)",
            Self::Add => "ADD (Repeating Key)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncodingType {
    #[default]
    None,
    Xor,
    Add,
    Sub,
}

impl EncodingType {
    pub const ALL: [Self; 4] = [Self::None, Self::Xor, Self::Add, Self::Sub];
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Xor => "XOR (1-Byte Key)",
            Self::Add => "ADD (1-Byte Key)",
            Self::Sub => "SUB (1-Byte Key)",
        }
    }
}

pub trait Encryptor {
    #[allow(dead_code)]
    fn name(&self) -> &'static str;
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8>;
    fn generate_stub(&self, key: &[u8], payload_len: usize, arch: TargetArch, att_syntax: bool) -> Result<String, String>;
}

pub trait Encoder {
    #[allow(dead_code)]
    fn name(&self) -> &'static str;
    fn encode(&self, data: &[u8], key: u8) -> Vec<u8>;
    fn generate_stub(&self, key: u8, payload_len: usize, arch: TargetArch, att_syntax: bool) -> Result<String, String>;
}

pub struct XorEncryptor;
pub struct AddEncryptor;

pub struct XorEncoder;
pub struct AddEncoder;
pub struct SubEncoder;

pub fn generate_mov_reg_nullfree(reg: &str, val: usize, att_syntax: bool) -> String {
    let parts = if let Some(p) = REGISTERS.iter().find(|p| p.r64 == reg || p.r32 == reg) {
        p
    } else {
        if att_syntax {
            return format!("    movl ${}, %{}\n", val, reg);
        } else {
            return format!("    mov {}, {}\n", reg, val);
        }
    };

    let r32 = parts.r32;
    let r16 = parts.r16;
    let r8 = parts.r8;

    if val == 0 {
        if att_syntax {
            format!("    xorl %{}, %{}\n", r32, r32)
        } else {
            format!("    xor {}, {}\n", r32, r32)
        }
    } else if val < 256 {
        if att_syntax {
            format!("    xorl %{}, %{}\n    movb ${}, %{}\n", r32, r32, val, r8)
        } else {
            format!("    xor {}, {}\n    mov {}, {}\n", r32, r32, r8, val)
        }
    } else if val < 65536 {
        let low = (val & 0xFF) as u8;
        let high = ((val >> 8) & 0xFF) as u8;
        if low != 0 && high != 0 {
            if att_syntax {
                format!("    xorl %{}, %{}\n    movw ${}, %{}\n", r32, r32, val, r16)
            } else {
                format!("    xor {}, {}\n    mov {}, {}\n", r32, r32, r16, val)
            }
        } else {
            let adj = val + 1;
            let adj_low = (adj & 0xFF) as u8;
            let adj_high = ((adj >> 8) & 0xFF) as u8;
            if adj_low != 0 && adj_high != 0 {
                if att_syntax {
                    format!("    xorl %{}, %{}\n    movw ${}, %{}\n    decl %{}\n", r32, r32, adj, r16, r32)
                } else {
                    format!("    xor {}, {}\n    mov {}, {}\n    dec {}\n", r32, r32, r16, adj, r32)
                }
            } else {
                if att_syntax {
                    format!("    movl ${}, %{}\n", val, r32)
                } else {
                    format!("    mov {}, {}\n", r32, val)
                }
            }
        }
    } else {
        if att_syntax {
            format!("    movl ${}, %{}\n", val, r32)
        } else {
            format!("    mov {}, {}\n", r32, val)
        }
    }
}

pub fn rle_compress(data: &[u8], marker: u8) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let byte = data[i];
        let mut run_len = 1;
        while i + run_len < data.len() && data[i + run_len] == byte && run_len < 255 {
            run_len += 1;
        }
        
        if byte == marker {
            compressed.push(marker);
            compressed.push(run_len as u8);
            compressed.push(marker);
        } else if run_len >= 3 {
            compressed.push(marker);
            compressed.push(run_len as u8);
            compressed.push(byte);
        } else {
            for _ in 0..run_len {
                compressed.push(byte);
            }
        }
        i += run_len;
    }
    compressed
}

pub fn find_best_rle_marker(data: &[u8], bad_chars: &HashSet<u8>) -> Option<u8> {
    let mut best_marker = None;
    let mut min_len = usize::MAX;
    
    for marker in 0..=255 {
        if bad_chars.contains(&marker) {
            continue;
        }
        let compressed = rle_compress(data, marker);
        if compressed.len() < min_len {
            min_len = compressed.len();
            best_marker = Some(marker);
        }
    }
    best_marker
}

pub fn generate_rle_stub(
    marker: u8,
    compressed_len: usize,
    _decompressed_len: usize,
    arch: TargetArch,
    att_syntax: bool,
) -> Result<String, String> {
    let (reg_suffix, inst_suffix) = match arch {
        TargetArch::X86_64 => ("r", "q"),
        TargetArch::X86 => ("e", "l"),
        _ => return Err(format!("RLE compression stub generation not supported for {}", arch.display_name())),
    };

    let temp_reg = match arch {
        TargetArch::X86_64 => "r8",
        TargetArch::X86 => "ebx",
        _ => unreachable!(),
    };

    let (prefix, suffix, b_suffix) = if att_syntax {
        ("%", inst_suffix, "b")
    } else {
        ("", "", "")
    };

    let si = format!("{}{}si", prefix, reg_suffix);
    let di = format!("{}{}di", prefix, reg_suffix);
    let sp = format!("{}{}sp", prefix, reg_suffix);
    let cx = format!("{}{}cx", prefix, reg_suffix);
    let temp = if att_syntax { format!("%{}", temp_reg) } else { temp_reg.to_string() };
    let al = format!("{}al", prefix);
    let dl = format!("{}dl", prefix);

    let pop = format!("pop{}", suffix);
    let push = format!("push{}", suffix);
    let mov = format!("mov{}", suffix);
    let sub = format!("sub{}", suffix);
    let add = format!("add{}", suffix);
    let dec = format!("dec{}", suffix);
    let test = format!("test{}", suffix);

    let cmp_marker = if att_syntax {
        format!("cmpb ${}, {}", marker, al)
    } else {
        format!("cmp {}, {}", al, marker)
    };

    let mov_al_dl = if att_syntax {
        format!("movb {}, {}", al, dl)
    } else {
        format!("mov {}, {}", dl, al)
    };

    let dec_dl = format!("dec{} {}", b_suffix, dl);

    let mov_si_temp = if att_syntax { format!("{} {}, {}", mov, si, temp) } else { format!("{} {}, {}", mov, temp, si) };
    let sub_cx_sp = if att_syntax { format!("{} {}, {}", sub, cx, sp) } else { format!("{} {}, {}", sub, sp, cx) };
    let mov_sp_di = if att_syntax { format!("{} {}, {}", mov, sp, di) } else { format!("{} {}, {}", mov, di, sp) };
    let mov_temp_di = if att_syntax { format!("{} {}, {}", mov, temp, di) } else { format!("{} {}, {}", mov, di, temp) };
    let test_cx_cx = format!("{} {}, {}", test, cx, cx);
    let add_cx_sp = if att_syntax { format!("{} {}, {}", add, cx, sp) } else { format!("{} {}, {}", add, sp, cx) };

    let mut s = String::new();
    s.push_str("jmp get_payload\n");
    s.push_str("decoder_stub:\n");
    s.push_str(&format!("    {} {}\n", pop, si));
    s.push_str(&format!("    {}\n", mov_si_temp));
    s.push_str(&format!("    {} {}\n", push, temp));
    s.push_str("    cld\n");
    s.push_str(&generate_mov_reg_nullfree(&format!("{}cx", reg_suffix), compressed_len, att_syntax));
    s.push_str(&format!("    {}\n", sub_cx_sp));
    s.push_str(&format!("    {}\n", mov_sp_di));
    s.push_str(&format!("    {} {}\n", push, di));
    s.push_str(&format!("    {} {}\n", push, cx));
    s.push_str("    rep movsb\n");
    s.push_str(&format!("    {} {}\n", pop, cx));
    s.push_str(&format!("    {} {}\n", pop, si));
    s.push_str(&format!("    {}\n", mov_temp_di));
    s.push_str("decompress_loop:\n");
    s.push_str(&format!("    {}\n", test_cx_cx));
    s.push_str("    jz decompress_done\n");
    s.push_str("    lodsb\n");
    s.push_str(&format!("    {} {}\n", dec, cx));
    s.push_str(&format!("    {}\n", cmp_marker));
    s.push_str("    jne write_literal\n");
    s.push_str("    lodsb\n");
    s.push_str(&format!("    {} {}\n", dec, cx));
    s.push_str(&format!("    {}\n", mov_al_dl));
    s.push_str("    lodsb\n");
    s.push_str(&format!("    {} {}\n", dec, cx));
    s.push_str("write_run_loop:\n");
    s.push_str("    stosb\n");
    s.push_str(&format!("    {}\n", dec_dl));
    s.push_str("    jnz write_run_loop\n");
    s.push_str("    jmp decompress_loop\n");
    s.push_str("write_literal:\n");
    s.push_str("    stosb\n");
    s.push_str("    jmp decompress_loop\n");
    s.push_str("decompress_done:\n");
    s.push_str(&generate_mov_reg_nullfree(&format!("{}cx", reg_suffix), compressed_len, att_syntax));
    s.push_str(&format!("    {}\n", add_cx_sp));
    s.push_str("    ret\n");
    s.push_str("get_payload:\n");
    s.push_str("    call decoder_stub\n");

    Ok(s)
}

#[allow(clippy::too_many_arguments)]
fn generate_repeating_key_stub(
    key: &[u8],
    payload_len: usize,
    arch: TargetArch,
    att_syntax: bool,
    op_att_64: &str,
    op_intel_64: &str,
    op_att_32: &str,
    op_intel_32: &str,
    encryptor_name: &str,
) -> Result<String, String> {
    if key.is_empty() {
        return Err("Encryption key cannot be empty".to_string());
    }

    let (reg_suffix, inst_suffix) = match arch {
        TargetArch::X86_64 => ("r", "q"),
        TargetArch::X86 => ("e", "l"),
        _ => return Err(format!("{} encryptor stub generation not supported for {}", encryptor_name, arch.display_name())),
    };

    let (key_start_reg, key_end_reg, key_byte_reg, op_code) = match arch {
        TargetArch::X86_64 => ("r8", "r9", "al", if att_syntax { op_att_64 } else { op_intel_64 }),
        TargetArch::X86 => ("edx", "eax", "bl", if att_syntax { op_att_32 } else { op_intel_32 }),
        _ => unreachable!(),
    };

    let (prefix, suffix) = if att_syntax {
        ("%", inst_suffix)
    } else {
        ("", "")
    };

    let si = format!("{}{}si", prefix, reg_suffix);
    let di = format!("{}{}di", prefix, reg_suffix);
    let cx = format!("{}{}cx", prefix, reg_suffix);
    
    let key_start = if att_syntax { format!("%{}", key_start_reg) } else { key_start_reg.to_string() };
    let key_end = if att_syntax { format!("%{}", key_end_reg) } else { key_end_reg.to_string() };
    let key_byte = if att_syntax { format!("%{}", key_byte_reg) } else { key_byte_reg.to_string() };

    let pop = format!("pop{}", suffix);
    let push = format!("push{}", suffix);
    let mov = format!("mov{}", suffix);
    let lea = format!("lea{}", suffix);
    let cmp = format!("cmp{}", suffix);
    let inc = format!("inc{}", suffix);
    let dec = format!("dec{}", suffix);

    let mov_si_start = if att_syntax { format!("{} {}, {}", mov, si, key_start) } else { format!("{} {}, {}", mov, key_start, si) };
    
    let lea_key_end = if att_syntax {
        format!("{} {}({}), {}", lea, key.len(), si, key_end)
    } else {
        format!("{} {}, [{} + {}]", lea, key_end, si, key.len())
    };

    let mov_end_di = if att_syntax { format!("{} {}, {}", mov, key_end, di) } else { format!("{} {}, {}", mov, di, key_end) };
    let cmp_end_si = if att_syntax { format!("{} {}, {}", cmp, key_end, si) } else { format!("{} {}, {}", cmp, si, key_end) };
    let mov_start_si = if att_syntax { format!("{} {}, {}", mov, key_start, si) } else { format!("{} {}, {}", mov, si, key_start) };
    
    let load_key_byte = if att_syntax {
        format!("movb ({}), {}", si, key_byte)
    } else {
        format!("mov {}, byte ptr [{}]", key_byte, si)
    };

    let mut s = String::new();
    s.push_str("jmp get_payload\n");
    s.push_str("decoder_stub:\n");
    s.push_str(&format!("    {} {}\n", pop, si));
    s.push_str(&format!("    {}\n", mov_si_start));
    s.push_str(&format!("    {}\n", lea_key_end));
    s.push_str(&format!("    {}\n", mov_end_di));
    s.push_str(&format!("    {} {}\n", push, di));
    s.push_str(&generate_mov_reg_nullfree(&format!("{}cx", reg_suffix), payload_len, att_syntax));
    s.push_str("decrypt_loop:\n");
    s.push_str(&format!("    {}\n", cmp_end_si));
    s.push_str("    jne key_ok\n");
    s.push_str(&format!("    {}\n", mov_start_si));
    s.push_str("key_ok:\n");
    s.push_str(&format!("    {}\n", load_key_byte));
    s.push_str(op_code);
    s.push_str(&format!("    {} {}\n", inc, si));
    s.push_str(&format!("    {} {}\n", inc, di));
    s.push_str(&format!("    {} {}\n", dec, cx));
    s.push_str("    jnz decrypt_loop\n");
    s.push_str("    ret\n");
    s.push_str("get_payload:\n");
    s.push_str("    call decoder_stub\n");
    s.push_str("    .byte ");
    s.push_str(&key.iter().map(|b| format!("0x{:02x}", b)).collect::<Vec<_>>().join(", "));
    s.push('\n');

    Ok(s)
}

#[allow(clippy::too_many_arguments)]
fn generate_1byte_stub(
    key: u8,
    payload_len: usize,
    arch: TargetArch,
    att_syntax: bool,
    op_att_64: &str,
    op_intel_64: &str,
    op_att_32: &str,
    op_intel_32: &str,
    encoder_name: &str,
) -> Result<String, String> {
    let (reg_suffix, inst_suffix) = match arch {
        TargetArch::X86_64 => ("r", "q"),
        TargetArch::X86 => ("e", "l"),
        _ => return Err(format!("{} encoder stub generation not supported for {}", encoder_name, arch.display_name())),
    };

    let op_code = match arch {
        TargetArch::X86_64 => if att_syntax { op_att_64 } else { op_intel_64 },
        TargetArch::X86 => if att_syntax { op_att_32 } else { op_intel_32 },
        _ => unreachable!(),
    };

    let (prefix, suffix) = if att_syntax {
        ("%", inst_suffix)
    } else {
        ("", "")
    };

    let si = format!("{}{}si", prefix, reg_suffix);
    let cx = format!("{}{}cx", prefix, reg_suffix);

    let pop = format!("pop{}", suffix);
    let push = format!("push{}", suffix);
    let inc = format!("inc{}", suffix);
    let dec = format!("dec{}", suffix);

    let decode_inst = if att_syntax {
        format!("{} ${}, ({})", op_code, key, si)
    } else {
        format!("{} byte ptr [{}], {}", op_code, si, key)
    };

    let mut s = String::new();
    s.push_str("jmp get_payload\n");
    s.push_str("decoder_stub:\n");
    s.push_str(&format!("    {} {}\n", pop, si));
    s.push_str(&format!("    {} {}\n", push, si));
    s.push_str(&generate_mov_reg_nullfree(&format!("{}cx", reg_suffix), payload_len, att_syntax));
    s.push_str("decode_loop:\n");
    s.push_str(&format!("    {}\n", decode_inst));
    s.push_str(&format!("    {} {}\n", inc, si));
    s.push_str(&format!("    {} {}\n", dec, cx));
    s.push_str("    jnz decode_loop\n");
    s.push_str("    ret\n");
    s.push_str("get_payload:\n");
    s.push_str("    call decoder_stub\n");

    Ok(s)
}

impl Encryptor for XorEncryptor {
    fn name(&self) -> &'static str {
        "XOR (Repeating Key)"
    }

    fn encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return data.to_vec();
        }
        data.iter().enumerate().map(|(i, &b)| b ^ key[i % key.len()]).collect()
    }

    fn generate_stub(&self, key: &[u8], payload_len: usize, arch: TargetArch, att_syntax: bool) -> Result<String, String> {
        generate_repeating_key_stub(
            key,
            payload_len,
            arch,
            att_syntax,
            "    xorb %al, (%rdi)\n",
            "    xor byte ptr [rdi], al\n",
            "    xorb %bl, (%edi)\n",
            "    xor byte ptr [edi], bl\n",
            "XOR",
        )
    }
}

impl Encryptor for AddEncryptor {
    fn name(&self) -> &'static str {
        "ADD (Repeating Key)"
    }

    fn encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return data.to_vec();
        }
        data.iter().enumerate().map(|(i, &b)| b.wrapping_add(key[i % key.len()])).collect()
    }

    fn generate_stub(&self, key: &[u8], payload_len: usize, arch: TargetArch, att_syntax: bool) -> Result<String, String> {
        generate_repeating_key_stub(
            key,
            payload_len,
            arch,
            att_syntax,
            "    subb %al, (%rdi)\n",
            "    sub byte ptr [rdi], al\n",
            "    subb %bl, (%edi)\n",
            "    sub byte ptr [edi], bl\n",
            "ADD",
        )
    }
}

impl Encoder for XorEncoder {
    fn name(&self) -> &'static str {
        "XOR (1-Byte Key)"
    }

    fn encode(&self, data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&b| b ^ key).collect()
    }

    fn generate_stub(&self, key: u8, payload_len: usize, arch: TargetArch, att_syntax: bool) -> Result<String, String> {
        generate_1byte_stub(
            key,
            payload_len,
            arch,
            att_syntax,
            "xorb",
            "xor",
            "xorb",
            "xor",
            "XOR",
        )
    }
}

impl Encoder for AddEncoder {
    fn name(&self) -> &'static str {
        "ADD (1-Byte Key)"
    }

    fn encode(&self, data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&b| b.wrapping_add(key)).collect()
    }

    fn generate_stub(&self, key: u8, payload_len: usize, arch: TargetArch, att_syntax: bool) -> Result<String, String> {
        generate_1byte_stub(
            key,
            payload_len,
            arch,
            att_syntax,
            "subb",
            "sub",
            "subb",
            "sub",
            "ADD",
        )
    }
}

impl Encoder for SubEncoder {
    fn name(&self) -> &'static str {
        "SUB (1-Byte Key)"
    }

    fn encode(&self, data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&b| b.wrapping_sub(key)).collect()
    }

    fn generate_stub(&self, key: u8, payload_len: usize, arch: TargetArch, att_syntax: bool) -> Result<String, String> {
        generate_1byte_stub(
            key,
            payload_len,
            arch,
            att_syntax,
            "addb",
            "add",
            "addb",
            "add",
            "SUB",
        )
    }
}

pub fn parse_key(input: &str) -> Vec<u8> {
    let input = input.trim();
    if input.is_empty() {
        return Vec::new();
    }
    if input.contains("0x") || input.contains("\\x") || input.contains(' ') || input.contains(',') || input.contains(';') {
        let cleaned = input
            .replace("\\x", " ")
            .replace("0x", " ")
            .replace(",", " ")
            .replace(";", " ");
        let bytes: Vec<u8> = cleaned
            .split_whitespace()
            .filter_map(|token| u8::from_str_radix(token, 16).ok())
            .collect();
        if !bytes.is_empty() {
            return bytes;
        }
    }
    if input.len() % 2 == 0 && input.chars().all(|c| c.is_ascii_hexdigit()) {
        return input
            .as_bytes()
            .chunks(2)
            .filter_map(|chunk| std::str::from_utf8(chunk).ok())
            .filter_map(|s| u8::from_str_radix(s, 16).ok())
            .collect();
    }
    input.as_bytes().to_vec()
}

pub fn find_best_encoding(
    encoder: &dyn Encoder,
    payload: &[u8],
    bad_chars: &HashSet<u8>,
    arch: TargetArch,
    att_syntax: bool,
) -> Result<(u8, Vec<u8>), String> {
    for key in (1..=255).chain(std::iter::once(0)) {
        if bad_chars.contains(&key) {
            continue;
        }
        let encoded = encoder.encode(payload, key);
        let stub_code = match encoder.generate_stub(key, payload.len(), arch, att_syntax) {
            Ok(code) => code,
            Err(_) => continue,
        };
        let stub_bytes = match crate::assembler::assemble(&stub_code, crate::debugger::CODE_BASE as u64, att_syntax, arch) {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };

        if stub_bytes.iter().any(|b| bad_chars.contains(b)) {
            continue;
        }

        if encoded.iter().any(|b| bad_chars.contains(b)) {
            continue;
        }

        let mut final_payload = stub_bytes;
        final_payload.extend_from_slice(&encoded);
        return Ok((key, final_payload));
    }

    Err("Failed to find a key that avoids bad characters".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key() {
        assert_eq!(parse_key("Hello"), vec![72, 101, 108, 108, 111]);
        assert_eq!(parse_key("ABCD"), vec![171, 205]);
        assert_eq!(parse_key("41424344"), vec![65, 66, 67, 68]);
        assert_eq!(parse_key("\\x41\\x42"), vec![65, 66]);
        assert_eq!(parse_key("0x41, 0x42"), vec![65, 66]);
    }

    #[test]
    fn test_xor_encryptor() {
        let encryptor = XorEncryptor;
        let data = vec![1, 2, 3, 4];
        let key = vec![10, 20];
        let encrypted = encryptor.encrypt(&data, &key);
        assert_eq!(encrypted, vec![1 ^ 10, 2 ^ 20, 3 ^ 10, 4 ^ 20]);
    }

    #[test]
    fn test_add_encryptor() {
        let encryptor = AddEncryptor;
        let data = vec![1, 2, 3, 4];
        let key = vec![10, 20];
        let encrypted = encryptor.encrypt(&data, &key);
        assert_eq!(encrypted, vec![11, 22, 13, 24]);
    }

    #[test]
    fn test_encoders() {
        let xor_enc = XorEncoder;
        assert_eq!(xor_enc.encode(&[1, 2, 3], 5), vec![1^5, 2^5, 3^5]);

        let add_enc = AddEncoder;
        assert_eq!(add_enc.encode(&[250, 255, 0], 10), vec![4, 9, 10]);

        let sub_enc = SubEncoder;
        assert_eq!(sub_enc.encode(&[10, 0, 5], 10), vec![0, 246, 251]);
    }

    #[test]
    fn test_find_best_encoding() {
        let payload = vec![0x90, 0x90, 0x90];
        let mut bad_chars = HashSet::new();
        bad_chars.insert(0x91);

        let encoder = XorEncoder;
        let res = find_best_encoding(&encoder, &payload, &bad_chars, TargetArch::X86_64, false);
        assert!(res.is_ok());
        let (key, encoded_payload) = res.unwrap();
        assert_ne!(key, 0x91);
        assert!(!encoded_payload.contains(&0x91));

        // Test with 0x00 as a bad character (must succeed now that stubs are null-free)
        let mut bad_chars_with_zero = HashSet::new();
        bad_chars_with_zero.insert(0x00);
        let res_zero = find_best_encoding(&encoder, &payload, &bad_chars_with_zero, TargetArch::X86_64, false);
        assert!(res_zero.is_ok(), "Failed to resolve encoding key when 0x00 is a bad character: {:?}", res_zero.err());
        let (key_zero, encoded_zero) = res_zero.unwrap();
        assert_ne!(key_zero, 0x00);
        assert!(!encoded_zero.contains(&0x00), "Encoded payload contains null bytes: {:?}", encoded_zero);
    }

    #[test]
    fn test_rle_compression() {
        let data = vec![0x90, 0x90, 0x90, 0x90, 0x00, 0x41, 0x41, 0x41, 0x00, 0x00];
        let marker = 0xcc;
        let compressed = rle_compress(&data, marker);
        // Expect: [marker, 4, 0x90, 0x00, marker, 3, 0x41, 0x00, 0x00]
        assert_eq!(
            compressed,
            vec![0xcc, 4, 0x90, 0x00, 0xcc, 3, 0x41, 0x00, 0x00]
        );
        
        let marker_in_data = vec![0xcc, 0xcc, 0xcc];
        let compressed_marker = rle_compress(&marker_in_data, marker);
        // Expect always encoded: [marker, 3, marker]
        assert_eq!(compressed_marker, vec![0xcc, 3, 0xcc]);
        
        let bad_chars = HashSet::new();
        let best_marker = find_best_rle_marker(&data, &bad_chars);
        assert!(best_marker.is_some());
        assert_ne!(best_marker.unwrap(), 0x90);
        assert_ne!(best_marker.unwrap(), 0x00);
        assert_ne!(best_marker.unwrap(), 0x41);
    }

    #[test]
    fn test_generate_mov_reg_nullfree() {
        let code_att = generate_mov_reg_nullfree("rcx", 100, true);
        assert!(code_att.contains("xorl %ecx, %ecx"));
        assert!(code_att.contains("movb $100, %cl"));

        let code_intel = generate_mov_reg_nullfree("rcx", 100, false);
        assert!(code_intel.contains("xor ecx, ecx"));
        assert!(code_intel.contains("mov cl, 100"));

        let code_zero = generate_mov_reg_nullfree("rcx", 0, false);
        assert!(code_zero.contains("xor ecx, ecx"));
        assert!(!code_zero.contains("mov"));
    }

    #[test]
    fn test_generate_rle_stub() {
        let archs = [TargetArch::X86_64, TargetArch::X86];
        for arch in archs {
            for att_syntax in [true, false] {
                let res = generate_rle_stub(0xcc, 20, 50, arch, att_syntax);
                assert!(res.is_ok());
                let stub = res.unwrap();
                let bytes = crate::assembler::assemble(&stub, crate::debugger::CODE_BASE as u64, att_syntax, arch);
                assert!(bytes.is_ok(), "Stub assembly failed for {:?}, AT&T={}: {:?}", arch, att_syntax, bytes.err());
            }
        }
    }
}
