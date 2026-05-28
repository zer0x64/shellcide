use aes::cipher::{BlockModeEncrypt, KeyInit};
use rand::Rng;

use crate::app::TargetArch;
use std::{collections::HashSet, io::Read};

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
    RegParts {
        r64: "rax",
        r32: "eax",
        r16: "ax",
        r8: "al",
    },
    RegParts {
        r64: "rcx",
        r32: "ecx",
        r16: "cx",
        r8: "cl",
    },
    RegParts {
        r64: "rdx",
        r32: "edx",
        r16: "dx",
        r8: "dl",
    },
    RegParts {
        r64: "rbx",
        r32: "ebx",
        r16: "bx",
        r8: "bl",
    },
    RegParts {
        r64: "rsi",
        r32: "esi",
        r16: "si",
        r8: "sil",
    },
    RegParts {
        r64: "rdi",
        r32: "edi",
        r16: "di",
        r8: "dil",
    },
    RegParts {
        r64: "rsp",
        r32: "esp",
        r16: "sp",
        r8: "spl",
    },
    RegParts {
        r64: "rbp",
        r32: "ebp",
        r16: "bp",
        r8: "bpl",
    },
    RegParts {
        r64: "r8",
        r32: "r8d",
        r16: "r8w",
        r8: "r8b",
    },
    RegParts {
        r64: "r9",
        r32: "r9d",
        r16: "r9w",
        r8: "r9b",
    },
    RegParts {
        r64: "r10",
        r32: "r10d",
        r16: "r10w",
        r8: "r10b",
    },
    RegParts {
        r64: "r11",
        r32: "r11d",
        r16: "r11w",
        r8: "r11b",
    },
    RegParts {
        r64: "r12",
        r32: "r12d",
        r16: "r12w",
        r8: "r12b",
    },
    RegParts {
        r64: "r13",
        r32: "r13d",
        r16: "r13w",
        r8: "r13b",
    },
    RegParts {
        r64: "r14",
        r32: "r14d",
        r16: "r14w",
        r8: "r14b",
    },
    RegParts {
        r64: "r15",
        r32: "r15d",
        r16: "r15w",
        r8: "r15b",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncryptionType {
    #[default]
    None,
    Xor,
    Add,
    Aes,
}

impl EncryptionType {
    pub const ALL: [Self; 4] = [Self::None, Self::Xor, Self::Add, Self::Aes];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Xor => "XOR (Repeating Key)",
            Self::Add => "ADD (Repeating Key)",
            Self::Aes => "AES (128-bit Key)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Xor => "XOR",
            Self::Add => "ADD",
            Self::Aes => "AES",
        }
    }

    pub fn get_encryptor(&self) -> Option<Box<dyn Encryptor>> {
        match self {
            Self::None => None,
            Self::Xor => Some(Box::new(XorEncryptor)),
            Self::Add => Some(Box::new(AddEncryptor)),
            Self::Aes => Some(Box::new(AesEncryptor)),
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

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Xor => "XOR",
            Self::Add => "ADD",
            Self::Sub => "SUB",
        }
    }

    pub fn get_encoder(&self) -> Option<Box<dyn Encoder>> {
        match self {
            Self::None => None,
            Self::Xor => Some(Box::new(XorEncoder)),
            Self::Add => Some(Box::new(AddEncoder)),
            Self::Sub => Some(Box::new(SubEncoder)),
        }
    }
}

pub trait Encryptor {
    #[allow(dead_code)]
    fn name(&self) -> &'static str;
    fn encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8>;
    fn generate_stub(
        &self,
        key: &[u8],
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String>;
}

pub trait Encoder {
    #[allow(dead_code)]
    fn name(&self) -> &'static str;
    fn encode(&self, data: &[u8], key: u8) -> Vec<u8>;
    fn generate_stub(
        &self,
        key: u8,
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String>;
}

pub struct XorEncryptor;
pub struct AddEncryptor;
pub struct AesEncryptor;

pub struct XorEncoder;
pub struct AddEncoder;
pub struct SubEncoder;

pub(crate) fn format_comma_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("0x{:02x}", b))
        .collect::<Vec<_>>()
        .join(", ")
}

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
                    format!(
                        "    xorl %{}, %{}\n    movw ${}, %{}\n    decl %{}\n",
                        r32, r32, adj, r16, r32
                    )
                } else {
                    format!(
                        "    xor {}, {}\n    mov {}, {}\n    dec {}\n",
                        r32, r32, r16, adj, r32
                    )
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
    let mut s = match arch {
        TargetArch::X86_64 => {
            let mut s = String::new();
            s.push_str("jmp get_payload\n");
            s.push_str("decoder_stub:\n");
            s.push_str("    pop rsi\n");
            s.push_str("    mov r8, rsi\n");
            s.push_str("    push r8\n");
            s.push_str("    cld\n");
            s.push_str(&generate_mov_reg_nullfree("rcx", compressed_len, false));
            s.push_str("    sub rsp, rcx\n");
            s.push_str("    mov rdi, rsp\n");
            s.push_str("    push rdi\n");
            s.push_str("    push rcx\n");
            s.push_str("    rep movsb\n");
            s.push_str("    pop rcx\n");
            s.push_str("    pop rsi\n");
            s.push_str("    mov rdi, r8\n");
            s.push_str("decompress_loop:\n");
            s.push_str("    test rcx, rcx\n");
            s.push_str("    jz decompress_done\n");
            s.push_str("    lodsb\n");
            s.push_str("    dec rcx\n");
            s.push_str(&format!("    cmp al, {}\n", marker));
            s.push_str("    jne write_literal\n");
            s.push_str("    lodsb\n");
            s.push_str("    dec rcx\n");
            s.push_str("    mov dl, al\n");
            s.push_str("    lodsb\n");
            s.push_str("    dec rcx\n");
            s.push_str("write_run_loop:\n");
            s.push_str("    stosb\n");
            s.push_str("    dec dl\n");
            s.push_str("    jnz write_run_loop\n");
            s.push_str("    jmp decompress_loop\n");
            s.push_str("write_literal:\n");
            s.push_str("    stosb\n");
            s.push_str("    jmp decompress_loop\n");
            s.push_str("decompress_done:\n");
            s.push_str(&generate_mov_reg_nullfree("rcx", compressed_len, false));
            s.push_str("    add rsp, rcx\n");
            s.push_str("    ret\n");
            s.push_str("get_payload:\n");
            s.push_str("    call decoder_stub\n");
            s
        }
        TargetArch::X86 => {
            let mut s = String::new();
            s.push_str("jmp get_payload\n");
            s.push_str("decoder_stub:\n");
            s.push_str("    pop esi\n");
            s.push_str("    mov ebx, esi\n");
            s.push_str("    push ebx\n");
            s.push_str("    cld\n");
            s.push_str(&generate_mov_reg_nullfree("ecx", compressed_len, false));
            s.push_str("    sub esp, ecx\n");
            s.push_str("    mov edi, esp\n");
            s.push_str("    push edi\n");
            s.push_str("    push ecx\n");
            s.push_str("    rep movsb\n");
            s.push_str("    pop ecx\n");
            s.push_str("    pop esi\n");
            s.push_str("    mov edi, ebx\n");
            s.push_str("decompress_loop:\n");
            s.push_str("    test ecx, ecx\n");
            s.push_str("    jz decompress_done\n");
            s.push_str("    lodsb\n");
            s.push_str("    dec ecx\n");
            s.push_str(&format!("    cmp al, {}\n", marker));
            s.push_str("    jne write_literal\n");
            s.push_str("    lodsb\n");
            s.push_str("    dec ecx\n");
            s.push_str("    mov dl, al\n");
            s.push_str("    lodsb\n");
            s.push_str("    dec ecx\n");
            s.push_str("write_run_loop:\n");
            s.push_str("    stosb\n");
            s.push_str("    dec dl\n");
            s.push_str("    jnz write_run_loop\n");
            s.push_str("    jmp decompress_loop\n");
            s.push_str("write_literal:\n");
            s.push_str("    stosb\n");
            s.push_str("    jmp decompress_loop\n");
            s.push_str("decompress_done:\n");
            s.push_str(&generate_mov_reg_nullfree("ecx", compressed_len, false));
            s.push_str("    add esp, ecx\n");
            s.push_str("    ret\n");
            s.push_str("get_payload:\n");
            s.push_str("    call decoder_stub\n");
            s
        }
        _ => {
            return Err(format!(
                "RLE compression stub generation not supported for {}",
                arch.display_name()
            ))
        }
    };
    if att_syntax {
        s = crate::syntax_converter::intel_to_att(&s);
    }
    Ok(s)
}

fn generate_mov_ecx_nullfree(val: usize, att_syntax: bool) -> String {
    if val == 0 {
        if att_syntax {
            "    xorl %ecx, %ecx\n".to_string()
        } else {
            "    xor ecx, ecx\n".to_string()
        }
    } else if val < 256 {
        if att_syntax {
            format!("    xorl %ecx, %ecx\n    movb ${}, %cl\n", val)
        } else {
            format!("    xor ecx, ecx\n    mov cl, {}\n", val)
        }
    } else if val < 65536 {
        let low = (val & 0xFF) as u8;
        let high = ((val >> 8) & 0xFF) as u8;
        if low != 0 && high != 0 {
            if att_syntax {
                format!("    xorl %ecx, %ecx\n    movw ${}, %cx\n", val)
            } else {
                format!("    xor ecx, ecx\n    mov cx, {}\n", val)
            }
        } else {
            let adj = val + 1;
            let adj_low = (adj & 0xFF) as u8;
            let adj_high = ((adj >> 8) & 0xFF) as u8;
            if adj_low != 0 && adj_high != 0 {
                if att_syntax {
                    format!(
                        "    xorl %ecx, %ecx\n    movw ${}, %cx\n    decl %ecx\n",
                        adj
                    )
                } else {
                    format!("    xor ecx, ecx\n    mov cx, {}\n    dec ecx\n", adj)
                }
            } else {
                if att_syntax {
                    format!("    movl ${}, %ecx\n", val)
                } else {
                    format!("    mov ecx, {}\n", val)
                }
            }
        }
    } else {
        if att_syntax {
            format!("    movl ${}, %ecx\n", val)
        } else {
            format!("    mov ecx, {}\n", val)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_repeating_key_stub(
    key: &[u8],
    payload_len: usize,
    arch: TargetArch,
    att_syntax: bool,
    op_intel_64: &str,
    op_intel_32: &str,
    encryptor_name: &str,
) -> Result<String, String> {
    if key.is_empty() {
        return Err("Encryption key cannot be empty".to_string());
    }
    let mut s = match arch {
        TargetArch::X86_64 => {
            let mut s = String::new();
            s.push_str("jmp get_payload\n");
            s.push_str("decoder_stub:\n");
            s.push_str("    pop rsi\n");
            s.push_str("    mov r8, rsi\n");
            s.push_str(&format!("    lea r9, [rsi + {}]\n", key.len()));
            s.push_str("    mov rdi, r9\n");
            s.push_str("    push rdi\n");
            s.push_str(&generate_mov_ecx_nullfree(payload_len, false));
            s.push_str("decrypt_loop:\n");
            s.push_str("    cmp rsi, r9\n");
            s.push_str("    jne key_ok\n");
            s.push_str("    mov rsi, r8\n");
            s.push_str("key_ok:\n");
            s.push_str("    mov al, byte ptr [rsi]\n");
            s.push_str(op_intel_64);
            s.push_str("    inc rsi\n");
            s.push_str("    inc rdi\n");
            s.push_str("    dec rcx\n");
            s.push_str("    jnz decrypt_loop\n");
            s.push_str("    ret\n");
            s.push_str("get_payload:\n");
            s.push_str("    call decoder_stub\n");
            s.push_str("    .byte ");
            s.push_str(&format_comma_hex(key));
            s.push('\n');
            s
        }
        TargetArch::X86 => {
            let mut s = String::new();
            s.push_str("jmp get_payload\n");
            s.push_str("decoder_stub:\n");
            s.push_str("    pop esi\n");
            s.push_str("    mov edx, esi\n");
            s.push_str(&format!("    lea eax, [esi + {}]\n", key.len()));
            s.push_str("    mov edi, eax\n");
            s.push_str("    push edi\n");
            s.push_str(&generate_mov_ecx_nullfree(payload_len, false));
            s.push_str("decrypt_loop:\n");
            s.push_str("    cmp esi, eax\n");
            s.push_str("    jne key_ok\n");
            s.push_str("    mov esi, edx\n");
            s.push_str("key_ok:\n");
            s.push_str("    mov bl, byte ptr [esi]\n");
            s.push_str(op_intel_32);
            s.push_str("    inc esi\n");
            s.push_str("    inc edi\n");
            s.push_str("    dec ecx\n");
            s.push_str("    jnz decrypt_loop\n");
            s.push_str("    ret\n");
            s.push_str("get_payload:\n");
            s.push_str("    call decoder_stub\n");
            s.push_str("    .byte ");
            s.push_str(&format_comma_hex(key));
            s.push('\n');
            s
        }
        _ => {
            return Err(format!(
                "{} encryptor stub generation not supported for {}",
                encryptor_name,
                arch.display_name()
            ))
        }
    };
    if att_syntax {
        s = crate::syntax_converter::intel_to_att(&s);
    }
    Ok(s)
}

#[allow(clippy::too_many_arguments)]
fn generate_1byte_stub(
    key: u8,
    payload_len: usize,
    arch: TargetArch,
    att_syntax: bool,
    op_intel_64: &str,
    op_intel_32: &str,
    encoder_name: &str,
) -> Result<String, String> {
    let mut s = match arch {
        TargetArch::X86_64 => {
            let mut s = String::new();
            s.push_str("jmp get_payload\n");
            s.push_str("decoder_stub:\n");
            s.push_str("    pop rsi\n");
            s.push_str("    push rsi\n");
            s.push_str(&generate_mov_ecx_nullfree(payload_len, false));
            s.push_str("decode_loop:\n");
            s.push_str(&format!("    {} byte ptr [rsi], {}\n", op_intel_64, key));
            s.push_str("    inc rsi\n");
            s.push_str("    dec rcx\n");
            s.push_str("    jnz decode_loop\n");
            s.push_str("    ret\n");
            s.push_str("get_payload:\n");
            s.push_str("    call decoder_stub\n");
            s
        }
        TargetArch::X86 => {
            let mut s = String::new();
            s.push_str("jmp get_payload\n");
            s.push_str("decoder_stub:\n");
            s.push_str("    pop esi\n");
            s.push_str("    push esi\n");
            s.push_str(&generate_mov_ecx_nullfree(payload_len, false));
            s.push_str("decode_loop:\n");
            s.push_str(&format!("    {} byte ptr [esi], {}\n", op_intel_32, key));
            s.push_str("    inc esi\n");
            s.push_str("    dec ecx\n");
            s.push_str("    jnz decode_loop\n");
            s.push_str("    ret\n");
            s.push_str("get_payload:\n");
            s.push_str("    call decoder_stub\n");
            s
        }
        _ => {
            return Err(format!(
                "{} encoder stub generation not supported for {}",
                encoder_name,
                arch.display_name()
            ))
        }
    };
    if att_syntax {
        s = crate::syntax_converter::intel_to_att(&s);
    }
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
        data.iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect()
    }

    fn generate_stub(
        &self,
        key: &[u8],
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String> {
        generate_repeating_key_stub(
            key,
            payload_len,
            arch,
            att_syntax,
            "    xor byte ptr [rdi], al\n",
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
        data.iter()
            .enumerate()
            .map(|(i, &b)| b.wrapping_add(key[i % key.len()]))
            .collect()
    }

    fn generate_stub(
        &self,
        key: &[u8],
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String> {
        generate_repeating_key_stub(
            key,
            payload_len,
            arch,
            att_syntax,
            "    sub byte ptr [rdi], al\n",
            "    sub byte ptr [edi], bl\n",
            "ADD",
        )
    }
}

impl Encryptor for AesEncryptor {
    fn name(&self) -> &'static str {
        "AES (128-bit Key)"
    }

    fn encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        // Get 16 bytes without failing
        let key = expand_key(key);

        // Pad data to multiple of 16 bytes with nops
        let mut data = data.to_vec();
        data.resize((data.len() + 15) & !15, 0x90);
        let data_len = data.len();

        type Aes = ecb::Encryptor<aes::Aes128>;
        let encryptor = Aes::new(&key.into());
        let _ =
            encryptor.encrypt_padded::<aes::cipher::block_padding::NoPadding>(&mut data, data_len);

        data
    }

    fn generate_stub(
        &self,
        key: &[u8],
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String> {
        // Get 16 bytes key without failing
        let key: [u8; 16] = expand_key(key);

        // Format key as little-endian hex for use in assembly
        let key_str = format_comma_hex(&key);
        let n_blocks = ((payload_len + 15) & !15) / 16;

        let mut s = match arch {
            TargetArch::X86_64 => {
                let mut s = String::new();
                s.push_str("jmp get_payload\n");
                // Key expansion helper
                s.push_str(
                    "
                key_expansion_128_helper:
                    pshufd xmm2, xmm2, 255
                    movdqa xmm3, xmm1
                    push 3
                    pop rcx
                Lhelper_loop:
                    palignr xmm3, xmm4, 12
                    pxor xmm1, xmm3
                    loop Lhelper_loop
                    pxor xmm1, xmm2
                    ret
                \n",
                );
                s.push_str(&format!(
                    "
                decoder_stub:
                    # Get pointer to encrypted payload
                    pop rsi;

                    # Clear xmm4 to zero for the palignr byte-shift inside the helper
                    pxor xmm4, xmm4

                    # Load original key (Encryption Round 0)
                    movdqu xmm1, [rsi]
                    add rsi, 16

                    # We want to return to the payload after decryption
                    push rsi

                    movdqa xmm5, xmm1       # xmm5 = Decryption Round 10 key
                    # Round 1 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x01
                    call key_expansion_128_helper
                    aesimc xmm6, xmm1       # xmm6 = Decryption Round 9 key
                    # Round 2 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x02
                    call key_expansion_128_helper
                    aesimc xmm7, xmm1       # xmm7 = Decryption Round 8 key
                    # Round 3 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x04
                    call key_expansion_128_helper
                    aesimc xmm8, xmm1       # xmm8 = Decryption Round 7 key
                    # Round 4 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x08
                    call key_expansion_128_helper
                    aesimc xmm9, xmm1       # xmm9 = Decryption Round 6 key
                    # Round 5 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x10
                    call key_expansion_128_helper
                    aesimc xmm10, xmm1      # xmm10 = Decryption Round 5 key
                    # Round 6 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x20
                    call key_expansion_128_helper
                    aesimc xmm11, xmm1      # xmm11 = Decryption Round 4 key
                    # Round 7 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x40
                    call key_expansion_128_helper
                    aesimc xmm12, xmm1      # xmm12 = Decryption Round 3 key
                    # Round 8 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x80
                    call key_expansion_128_helper
                    aesimc xmm13, xmm1      # xmm13 = Decryption Round 2 key
                    # Round 9 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x1b
                    call key_expansion_128_helper
                    aesimc xmm14, xmm1      # xmm14 = Decryption Round 1 key
                    # Round 10 Key Expansion
                    aeskeygenassist xmm2, xmm1, 0x36
                    call key_expansion_128_helper
                    movdqa xmm15, xmm1      # xmm15 = Decryption Round 0 key (no aesimc)

                    mov rcx, {n_blocks}
                Lblock_loop:
                    # Load 16-byte ciphertext block
                    movdqu xmm0, [rsi]
                    # Initial XOR step with Decryption Round 0 Key
                    pxor xmm0, xmm15
                    # 9 unrolled intermediate rounds using the keys stored in registers
                    aesdec xmm0, xmm14
                    aesdec xmm0, xmm13
                    aesdec xmm0, xmm12
                    aesdec xmm0, xmm11
                    aesdec xmm0, xmm10
                    aesdec xmm0, xmm9
                    aesdec xmm0, xmm8
                    aesdec xmm0, xmm7
                    aesdec xmm0, xmm6
                    # Final round of aesdeclast using Decryption Round 10 Key
                    aesdeclast xmm0, xmm5
                    # Store decrypted plaintext block
                    movdqu [rsi], xmm0
                    add rsi, 16
                    dec rcx
                    jnz Lblock_loop
                "
                ));

                s.push_str("    ret\n");
                s.push_str("get_payload:\n");
                s.push_str("    call decoder_stub\n");
                s.push_str(&format!("    .db {}\n", key_str));
                s
            }
            _ => {
                return Err("Unsupported architecture".to_string());
            }
        };

        if att_syntax {
            s = crate::syntax_converter::intel_to_att(&s);
        }
        Ok(s)
    }
}

impl Encoder for XorEncoder {
    fn name(&self) -> &'static str {
        "XOR (1-Byte Key)"
    }

    fn encode(&self, data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&b| b ^ key).collect()
    }

    fn generate_stub(
        &self,
        key: u8,
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String> {
        generate_1byte_stub(key, payload_len, arch, att_syntax, "xor", "xor", "XOR")
    }
}

impl Encoder for AddEncoder {
    fn name(&self) -> &'static str {
        "ADD (1-Byte Key)"
    }

    fn encode(&self, data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&b| b.wrapping_add(key)).collect()
    }

    fn generate_stub(
        &self,
        key: u8,
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String> {
        generate_1byte_stub(key, payload_len, arch, att_syntax, "sub", "sub", "ADD")
    }
}

impl Encoder for SubEncoder {
    fn name(&self) -> &'static str {
        "SUB (1-Byte Key)"
    }

    fn encode(&self, data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&b| b.wrapping_sub(key)).collect()
    }

    fn generate_stub(
        &self,
        key: u8,
        payload_len: usize,
        arch: TargetArch,
        att_syntax: bool,
    ) -> Result<String, String> {
        generate_1byte_stub(key, payload_len, arch, att_syntax, "add", "add", "SUB")
    }
}

pub fn parse_key(input: &str) -> Vec<u8> {
    let input = input.trim();
    if input.is_empty() {
        return Vec::new();
    }
    if input.contains("0x") || input.contains("\\x") || input.contains(' ') || input.contains(',') {
        let mut bytes = Vec::new();
        let cleaned = input
            .replace("\\x", " ")
            .replace("0x", " ")
            .replace(",", " ")
            .replace(";", " ");
        for token in cleaned.split_whitespace() {
            if let Ok(b) = u8::from_str_radix(token, 16) {
                bytes.push(b);
            }
        }
        if !bytes.is_empty() {
            return bytes;
        }
    }
    if input.len().is_multiple_of(2) && input.chars().all(|c| c.is_ascii_hexdigit()) {
        let mut bytes = Vec::new();
        for chunk in input.as_bytes().chunks(2) {
            if let Ok(s) = std::str::from_utf8(chunk) {
                if let Ok(b) = u8::from_str_radix(s, 16) {
                    bytes.push(b);
                }
            }
        }
        return bytes;
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
        let stub_bytes = match crate::assembler::assemble(
            &stub_code,
            crate::debugger::CODE_BASE as u64,
            att_syntax,
            arch,
        ) {
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

/// Expands a key to the specified length using BLAKE3 hashing.
/// Useful for allowing users to specify a key of any length.
fn expand_key<const N: usize>(key: &[u8]) -> [u8; N] {
    let mut buf = [0u8; N];

    if key.is_empty() {
        // If no key is specified, generate a random key of length n
        let mut rng = rand::rng();
        rng.fill_bytes(&mut buf);
    } else if key.len() != N {
        // If the key is not the correct length, hash it to get a key of length n
        let mut hasher = blake3::Hasher::new();
        let mut hash = hasher.update(key).finalize_xof();

        hash.read_exact(&mut buf).expect("blake3 XOF exhausted");
    } else {
        // If the key is the correct length, use it as-is
        buf.copy_from_slice(key);
    }

    buf
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
        assert_eq!(xor_enc.encode(&[1, 2, 3], 5), vec![1 ^ 5, 2 ^ 5, 3 ^ 5]);

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
        let res_zero = find_best_encoding(
            &encoder,
            &payload,
            &bad_chars_with_zero,
            TargetArch::X86_64,
            false,
        );
        assert!(
            res_zero.is_ok(),
            "Failed to resolve encoding key when 0x00 is a bad character: {:?}",
            res_zero.err()
        );
        let (key_zero, encoded_zero) = res_zero.unwrap();
        assert_ne!(key_zero, 0x00);
        assert!(
            !encoded_zero.contains(&0x00),
            "Encoded payload contains null bytes: {:?}",
            encoded_zero
        );
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
                let bytes = crate::assembler::assemble(
                    &stub,
                    crate::debugger::CODE_BASE as u64,
                    att_syntax,
                    arch,
                );
                assert!(
                    bytes.is_ok(),
                    "Stub assembly failed for {:?}, AT&T={}: {:?}",
                    arch,
                    att_syntax,
                    bytes.err()
                );
            }
        }
    }
}
