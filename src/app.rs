use eframe::egui;
use flume::{Receiver, Sender};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::assembler::assemble;
use crate::encoder::Encryptor;
use crate::debugger::{CpuRegisters, DebuggerCommand, DebuggerEvent, Pid, CODE_BASE, DATA_BASE};
use crate::disassembler::{disassemble_code, DisassembledInstruction};
use crate::ui::LeftBottomTab;
use crate::ui::struct_packer::StructPackerState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetArch {
    #[default]
    X86_64,
    X86,
    Arm,
    Thumb,
    Aarch64,
    Riscv,
}

impl TargetArch {
    pub const ALL: [TargetArch; 6] = [
        TargetArch::X86_64,
        TargetArch::X86,
        TargetArch::Arm,
        TargetArch::Thumb,
        TargetArch::Aarch64,
        TargetArch::Riscv,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            TargetArch::X86_64 => "x86_64",
            TargetArch::X86 => "x86",
            TargetArch::Arm => "ARM",
            TargetArch::Thumb => "Thumb",
            TargetArch::Aarch64 => "AArch64",
            TargetArch::Riscv => "RISC-V",
        }
    }
}

pub struct ShellcideApp {
    // Code input and settings
    pub(crate) code_input: String,
    pub(crate) att_syntax: bool,
    pub(crate) target_arch: TargetArch,
    pub(crate) active_path: String,

    // Thread communication
    pub(crate) cmd_tx: Sender<DebuggerCommand>,
    #[allow(dead_code)]
    pub(crate) event_tx: Sender<DebuggerEvent>,
    pub(crate) event_rx: Receiver<DebuggerEvent>,
    pub(crate) shared_pid: Arc<Mutex<Option<Pid>>>,

    // Debugger state
    pub(crate) is_running: bool,
    pub(crate) is_stopped: bool,
    pub(crate) regs: CpuRegisters,
    pub(crate) previous_regs: Option<CpuRegisters>,
    pub(crate) disassembly: Vec<DisassembledInstruction>,
    pub(crate) breakpoints: HashSet<usize>,
    pub(crate) editor_breakpoints: HashSet<usize>,
    pub(crate) compiled_bytes: Vec<u8>,

    // Captured logs
    pub(crate) stdout_log: String,
    pub(crate) stderr_log: String,
    pub(crate) console_log: String,
    pub(crate) active_tab: ConsoleTab,

    // Memory Editor state
    pub(crate) mem_base_input: String,
    pub(crate) memory_base_address: usize,
    pub(crate) memory_data: Vec<u8>,

    // Inline editors
    pub(crate) editing_register: Option<String>,
    pub(crate) register_input: String,
    pub(crate) editing_memory_byte: Option<usize>,
    pub(crate) memory_byte_input: String,
    pub(crate) syscall_search: String,
    pub(crate) instructions_search: String,
    pub(crate) bad_chars_input: String,
    pub(crate) bad_char_lines: std::collections::HashSet<usize>,
    pub(crate) reg_change_times: std::collections::HashMap<String, std::time::Instant>,
    pub(crate) auto_follow_rsp: bool,

    // Struct Packer and bottom-left tab
    pub(crate) left_bottom_tab: LeftBottomTab,
    pub(crate) struct_packer: StructPackerState,

    // Post-processing Pipeline settings
    pub(crate) compression_type: crate::encoder::CompressionType,
    pub(crate) resolved_compression_marker: Option<u8>,
    pub(crate) encryption_type: crate::encoder::EncryptionType,
    pub(crate) encryption_key: String,
    pub(crate) encoding_type: crate::encoder::EncodingType,
    pub(crate) resolved_encoding_key: Option<u8>,
    pub(crate) breakpoint_originals: std::collections::HashMap<usize, u8>,
}

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
    Shellcode,
}

fn format_hex_escapes(bytes: &[u8], chunk_size: usize, line_prefix: &str) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let num_prefixes = (bytes.len() - 1) / chunk_size;
    let capacity = bytes.len() * 4 + num_prefixes * line_prefix.len();
    let mut out = String::with_capacity(capacity);
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && i % chunk_size == 0 {
            out.push_str(line_prefix);
        }
        use std::fmt::Write;
        let _ = write!(out, "\\x{:02x}", b);
    }
    out
}

impl ShellcideApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        cmd_tx: Sender<DebuggerCommand>,
        event_tx: Sender<DebuggerEvent>,
        event_rx: Receiver<DebuggerEvent>,
        shared_pid: Arc<Mutex<Option<Pid>>>,
    ) -> Self {
        crate::ui::theme::apply_cyber_cyan_theme(&_cc.egui_ctx);

        let default_code = crate::assembler::DEMO_CODE;

        Self {
            code_input: default_code.to_string(),
            att_syntax: false,
            target_arch: TargetArch::X86_64,
            active_path: "demo.s".to_string(),
            cmd_tx,
            event_tx,
            event_rx,
            shared_pid,
            is_running: false,
            is_stopped: false,
            regs: CpuRegisters::default(),
            previous_regs: None,
            disassembly: Vec::new(),
            breakpoints: HashSet::new(),
            editor_breakpoints: HashSet::new(),
            stdout_log: String::new(),
            stderr_log: String::new(),
            console_log: "System initialized.\nWelcome to Shellcide. Type shellcode assembly and click 'Assemble'.\n".to_string(),
            active_tab: ConsoleTab::Console,
            compiled_bytes: Vec::new(),
            mem_base_input: format!("0x{:X}", DATA_BASE),
            memory_base_address: DATA_BASE,
            memory_data: vec![0; 256],
            editing_register: None,
            register_input: String::new(),
            editing_memory_byte: None,
            memory_byte_input: String::new(),
            syscall_search: String::new(),
            instructions_search: String::new(),
            bad_chars_input: "00".to_string(),
            bad_char_lines: HashSet::new(),
            reg_change_times: HashMap::new(),
            auto_follow_rsp: false,
            left_bottom_tab: LeftBottomTab::Syscalls,
            struct_packer: StructPackerState::default(),
            compression_type: crate::encoder::CompressionType::None,
            resolved_compression_marker: None,
            encryption_type: crate::encoder::EncryptionType::None,
            encryption_key: String::new(),
            encoding_type: crate::encoder::EncodingType::None,
            resolved_encoding_key: None,
            breakpoint_originals: std::collections::HashMap::new(),
        }
    }

    pub(crate) fn log(&mut self, msg: &str) {
        self.console_log.push_str(msg);
        self.console_log.push('\n');
    }

    pub(crate) fn get_line_to_inst_mapping(&self) -> HashMap<usize, usize> {
        self.code_input
            .lines()
            .enumerate()
            .filter_map(|(line_idx, line)| {
                let trimmed = line.trim();
                let is_comment = trimmed.starts_with(';') || trimmed.starts_with('#');
                let is_label = trimmed.ends_with(':') && !trimmed.contains(' ');
                if !trimmed.is_empty() && !is_comment && !is_label {
                    Some(line_idx)
                } else {
                    None
                }
            })
            .enumerate()
            .map(|(inst_idx, line_idx)| (line_idx, inst_idx))
            .collect()
    }

    pub(crate) fn get_inst_to_line_mapping(&self) -> HashMap<usize, usize> {
        self.get_line_to_inst_mapping()
            .into_iter()
            .map(|(line_idx, inst_idx)| (inst_idx, line_idx))
            .collect()
    }

    pub(crate) fn sync_breakpoints_from_code(&mut self) {
        if self.disassembly.is_empty() {
            return;
        }
        self.breakpoints.clear();
        let line_to_inst_idx = self.get_line_to_inst_mapping();
        for &line_idx in &self.editor_breakpoints {
            if let Some(&inst_idx) = line_to_inst_idx.get(&line_idx) {
                if inst_idx < self.disassembly.len() {
                    let addr = self.disassembly[inst_idx].address as usize;
                    self.breakpoints.insert(addr);
                }
            }
        }
    }

    pub(crate) fn sync_code_from_breakpoints(&mut self) {
        if self.disassembly.is_empty() {
            return;
        }
        self.editor_breakpoints.clear();
        let inst_to_line = self.get_inst_to_line_mapping();
        for &addr in &self.breakpoints {
            if let Some(inst_idx) = self
                .disassembly
                .iter()
                .position(|inst| inst.address as usize == addr)
            {
                if let Some(&line_idx) = inst_to_line.get(&inst_idx) {
                    self.editor_breakpoints.insert(line_idx);
                }
            }
        }
    }

    pub(crate) fn run_post_processing_pipeline(&mut self, raw_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let mut current_payload = raw_bytes.to_vec();

        // 0. Compression Stage
        if self.compression_type == crate::encoder::CompressionType::None {
            self.resolved_compression_marker = None;
        } else {
            let bad_chars = crate::assembler::parse_bad_characters(&self.bad_chars_input);
            let marker = match crate::encoder::find_best_rle_marker(&current_payload, &bad_chars) {
                Some(m) => m,
                None => return Err("Failed to find a suitable RLE marker that avoids bad characters".to_string()),
            };
            self.resolved_compression_marker = Some(marker);
            
            let compressed = crate::encoder::rle_compress(&current_payload, marker);
            let stub_code = crate::encoder::generate_rle_stub(marker, compressed.len(), current_payload.len(), self.target_arch, self.att_syntax)?;
            let stub_bytes = assemble(&stub_code, CODE_BASE as u64, self.att_syntax, self.target_arch)?;
            
            let mut next_payload = stub_bytes;
            next_payload.extend_from_slice(&compressed);
            let stub_size = next_payload.len() - compressed.len();
            current_payload = next_payload;
            self.log(&format!("[✓] Compressed with RLE. Marker: 0x{:02X}, Original: {} bytes, Compressed: {} bytes. Stub size: {} bytes.", marker, raw_bytes.len(), compressed.len(), stub_size));
        }

        // 1. Encryption Stage
        if self.encryption_type != crate::encoder::EncryptionType::None {
            let key = crate::encoder::parse_key(&self.encryption_key);
            let name = match self.encryption_type {
                crate::encoder::EncryptionType::None => unreachable!(),
                crate::encoder::EncryptionType::Xor => "XOR",
                crate::encoder::EncryptionType::Add => "ADD",
            };
            if key.is_empty() {
                return Err(format!("{} encryption key is empty or invalid", name));
            }
            let encryptor: Box<dyn Encryptor> = match self.encryption_type {
                crate::encoder::EncryptionType::None => unreachable!(),
                crate::encoder::EncryptionType::Xor => Box::new(crate::encoder::XorEncryptor),
                crate::encoder::EncryptionType::Add => Box::new(crate::encoder::AddEncryptor),
            };
            let encrypted = encryptor.encrypt(&current_payload, &key);
            let stub_code = encryptor.generate_stub(&key, encrypted.len(), self.target_arch, self.att_syntax)?;
            let stub_bytes = assemble(&stub_code, CODE_BASE as u64, self.att_syntax, self.target_arch)?;
            
            let mut next_payload = stub_bytes;
            next_payload.extend_from_slice(&encrypted);
            let stub_size = next_payload.len() - encrypted.len();
            current_payload = next_payload;
            self.log(&format!("[+] Encrypted with {}. Stub size: {} bytes.", name, stub_size));
        }

        // 2. Encoding Stage
        if self.encoding_type == crate::encoder::EncodingType::None {
            self.resolved_encoding_key = None;
        } else {
            let name = match self.encoding_type {
                crate::encoder::EncodingType::None => unreachable!(),
                crate::encoder::EncodingType::Xor => "XOR",
                crate::encoder::EncodingType::Add => "ADD",
                crate::encoder::EncodingType::Sub => "SUB",
            };
            let bad_chars = crate::assembler::parse_bad_characters(&self.bad_chars_input);
            let encoder: Box<dyn crate::encoder::Encoder> = match self.encoding_type {
                crate::encoder::EncodingType::None => unreachable!(),
                crate::encoder::EncodingType::Xor => Box::new(crate::encoder::XorEncoder),
                crate::encoder::EncodingType::Add => Box::new(crate::encoder::AddEncoder),
                crate::encoder::EncodingType::Sub => Box::new(crate::encoder::SubEncoder),
            };
            let (key, encoded_payload) = crate::encoder::find_best_encoding(
                &*encoder,
                &current_payload,
                &bad_chars,
                self.target_arch,
                self.att_syntax,
            )?;
            current_payload = encoded_payload;
            self.resolved_encoding_key = Some(key);
            self.log(&format!("[✓] Encoded with {}. Resolved key: 0x{:02X}", name, key));
        }

        Ok(current_payload)
    }

    pub(crate) fn do_assemble(&mut self) {
        self.bad_char_lines.clear();
        self.log("[+] Assembling shellcode...");
        match assemble(
            &self.code_input,
            CODE_BASE as u64,
            self.att_syntax,
            self.target_arch,
        ) {
            Ok(bytes) => {
                self.log(&format!(
                    "[✓] Shellcode compiled successfully. Size: {} bytes.",
                    bytes.len()
                ));

                // Run post-processing pipeline
                let pipeline_res = self.run_post_processing_pipeline(&bytes);
                let final_bytes = match pipeline_res {
                    Ok(final_b) => final_b,
                    Err(e) => {
                        self.log(&format!("[✗] Post-processing pipeline failed: {}", e));
                        return;
                    }
                };

                let bad_chars = crate::assembler::parse_bad_characters(&self.bad_chars_input);
                if !bad_chars.is_empty() {
                    let mut found_bad_bytes = Vec::new();
                    for (i, &b) in final_bytes.iter().enumerate() {
                        if bad_chars.contains(&b) {
                            let addr = CODE_BASE + i;
                            found_bad_bytes.push(format!("0x{:02X} at 0x{:08X}", b, addr));
                        }
                    }
                    if !found_bad_bytes.is_empty() {
                        self.log(&format!(
                            "[⚠️ WARNING] Found {} bad character(s) in final payload:\n    {}",
                            found_bad_bytes.len(),
                            found_bad_bytes.join(", ")
                        ));
                    }
                }

                self.compiled_bytes = final_bytes.clone();
                self.disassembly =
                    disassemble_code(&final_bytes, CODE_BASE as u64, self.att_syntax, self.target_arch);

                // Calculate which lines contain bad characters (only if no encoding/encryption was done,
                // or mapped to disassembled instructions if possible)
                if !bad_chars.is_empty() {
                    let inst_to_line = self.get_inst_to_line_mapping();
                    for (inst_idx, inst) in self.disassembly.iter().enumerate() {
                        if inst.bytes.iter().any(|b| bad_chars.contains(b)) {
                            if let Some(&line_idx) = inst_to_line.get(&inst_idx) {
                                self.bad_char_lines.insert(line_idx);
                            }
                        }
                    }
                }

                // Sync breakpoints from editor text comments to our internal set
                self.sync_breakpoints_from_code();

                // Clear state
                self.is_running = false;
                self.is_stopped = false;
                self.previous_regs = None;
                self.regs = CpuRegisters::default();
                self.stdout_log.clear();
                self.stderr_log.clear();
            }
            Err(e) => {
                self.log(&format!("[✗] Shellcode assembly failed:\n{}", e));
            }
        }
    }

    pub(crate) fn refresh_memory(&mut self) {
        if let Some(pid) = *self.shared_pid.lock().unwrap() {
            if let Ok(data) = crate::debugger::read_child_mem(pid, self.memory_base_address, 256) {
                self.memory_data = data;
            }
        } else {
            // Fill with zero if no process is running
            self.memory_data = vec![0; 256];
        }
    }

    pub(crate) fn refresh_disassembly_from_running_process(&mut self) {
        if self.is_native_debug() {
            if let Some(pid) = *self.shared_pid.lock().unwrap() {
                let len = self.compiled_bytes.len();
                if len > 0 {
                    if let Ok(mut data) = crate::debugger::read_child_mem(pid, CODE_BASE, len) {
                        for (&addr, &orig_byte) in &self.breakpoint_originals {
                            if addr >= CODE_BASE && addr < CODE_BASE + len {
                                data[addr - CODE_BASE] = orig_byte;
                            }
                        }
                        self.disassembly = disassemble_code(&data, CODE_BASE as u64, self.att_syntax, self.target_arch);
                    }
                }
            }
        }
    }

    pub(crate) fn toggle_breakpoint(&mut self, addr: usize) {
        let is_native = self.is_native_debug();
        if !is_native {
            return;
        }

        let was_present = self.breakpoints.remove(&addr);
        if !was_present {
            self.breakpoints.insert(addr);

            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Some(pid) = *self.shared_pid.lock().unwrap() {
                    if let Ok(orig) = crate::debugger::read_child_mem(pid, addr, 1) {
                        self.breakpoint_originals.insert(addr, orig[0]);
                    }
                }
            }
        } else {
            self.breakpoint_originals.remove(&addr);
        }

        self.cmd_tx
            .send(crate::debugger::DebuggerCommand::ToggleBreakpoint(addr, !was_present))
            .ok();

        let action = if was_present { "Removed" } else { "Added" };
        self.log(&format!("[Breakpoint] {} at 0x{:08X}", action, addr));
    }

    pub(crate) fn jump_to_memory_address(&mut self, addr: usize) {
        self.memory_base_address = addr;
        self.mem_base_input = format!("0x{:X}", addr);
        self.refresh_memory();
    }

    pub(crate) fn get_centered_rsp(&self) -> usize {
        (self.regs.rsp as usize).saturating_sub(0x80) & !0xF
    }

    pub(crate) fn is_native_debug(&self) -> bool {
        !cfg!(target_arch = "wasm32") && self.target_arch == TargetArch::X86_64
    }

    pub(crate) fn format_raw_hex(&self, bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    pub(crate) fn format_python(&self, bytes: &[u8]) -> String {
        if bytes.is_empty() {
            return "shellcode = b\"\"".to_string();
        }
        format!(
            "shellcode = b\"\"\nshellcode += b\"{}\"",
            format_hex_escapes(bytes, 16, "\"\nshellcode += b\"")
        )
    }

    pub(crate) fn format_c(&self, bytes: &[u8]) -> String {
        if bytes.is_empty() {
            return "unsigned char shellcode[] = \"\";".to_string();
        }
        format!(
            "unsigned char shellcode[] = \n\"{}\";",
            format_hex_escapes(bytes, 16, "\"\n\"")
        )
    }

    pub(crate) fn format_rust(&self, bytes: &[u8]) -> String {
        if bytes.is_empty() {
            return "const SHELLCODE: &[u8] = &[];".to_string();
        }
        let capacity = 32 + bytes.len() * 6 + (bytes.len() / 12) * 5 + 4;
        let mut out = String::with_capacity(capacity);
        out.push_str("const SHELLCODE: &[u8] = &[\n    ");
        use std::fmt::Write;
        for (i, chunk) in bytes.chunks(12).enumerate() {
            if i > 0 {
                out.push_str("\n    ");
            }
            for &b in chunk {
                let _ = write!(out, "0x{:02x}, ", b);
            }
        }
        out.push_str("\n];");
        out
    }

    pub(crate) fn insert_into_editor(&mut self, ctx: &egui::Context, text: &str) {
        let text_edit_id = egui::Id::new("code_editor_text_edit");
        let mut state =
            egui::widgets::text_edit::TextEditState::load(ctx, text_edit_id).unwrap_or_default();
        let char_range = state.cursor.char_range();

        let inserted_len = text.chars().count();

        let (byte_idx, char_idx) = if let Some(range) = char_range {
            let cursor_idx = range.primary.index;
            let byte_offset = self
                .code_input
                .char_indices()
                .nth(cursor_idx)
                .map(|(i, _)| i)
                .unwrap_or(self.code_input.len());
            (byte_offset, cursor_idx)
        } else {
            let total_chars = self.code_input.chars().count();
            (self.code_input.len(), total_chars)
        };

        self.code_input.insert_str(byte_idx, text);

        // Update cursor position to end of inserted text
        let new_char_idx = char_idx + inserted_len;
        let new_range = egui::text::CCursorRange::two(
            egui::text::CCursor::new(new_char_idx),
            egui::text::CCursor::new(new_char_idx),
        );
        state.cursor.set_char_range(Some(new_range));
        state.store(ctx, text_edit_id);
    }

    pub(crate) fn update_register_change_timestamps(
        &mut self,
        prev: CpuRegisters,
        next: CpuRegisters,
    ) {
        let now = std::time::Instant::now();
        for name in [
            "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp", "rip", "rflags", "r8", "r9",
            "r10", "r11", "r12", "r13", "r14", "r15",
        ] {
            if prev.get_by_name(name) != next.get_by_name(name) {
                self.reg_change_times.insert(name.to_string(), now);
            }
        }
    }

    pub(crate) fn load_file(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        match std::fs::read_to_string(&self.active_path) {
            Ok(content) => {
                self.code_input = content;
                self.log(&format!("[File] Loaded file: {}", self.active_path));
            }
            Err(e) => self.log(&format!("[File System Error] Failed to load: {}", e)),
        }
        #[cfg(target_arch = "wasm32")]
        self.load_file_wasm();
    }

    pub(crate) fn save_file(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        match std::fs::write(&self.active_path, &self.code_input) {
            Ok(_) => self.log(&format!("[File] Saved to: {}", self.active_path)),
            Err(e) => self.log(&format!("[File System Error] Failed to save: {}", e)),
        }
        #[cfg(target_arch = "wasm32")]
        self.save_file_wasm();
    }

    #[cfg(target_arch = "wasm32")]
    fn save_file_wasm(&self) {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let array = js_sys::Array::new();
        array.push(&JsValue::from_str(&self.code_input));
        let blob = web_sys::Blob::new_with_str_sequence(&array).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        let link = document.create_element("a").unwrap();
        let html_link = link.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        html_link.set_href(&url);
        html_link.set_download(&self.active_path);
        let body = document.body().unwrap();
        body.append_child(&html_link).unwrap();
        html_link.click();
        body.remove_child(&html_link).unwrap();
        let _ = web_sys::Url::revoke_object_url(&url);
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file_wasm(&self) {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let input = document.create_element("input").unwrap();
        let html_input = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
        html_input.set_type("file");
        html_input.set_accept(".s,.asm,.txt,.bin,*");
        let tx_clone = self.event_tx.clone();
        let input_clone = html_input.clone();
        let on_change = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(files) = input_clone.files() {
                if let Some(file) = files.get(0) {
                    let filename = file.name();
                    let file_reader = web_sys::FileReader::new().unwrap();
                    let tx_inner = tx_clone.clone();
                    let file_reader_clone = file_reader.clone();
                    let filename_clone = filename.clone();
                    let on_load = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        if let Ok(result) = file_reader_clone.result() {
                            if let Some(content) = result.as_string() {
                                let _ = tx_inner.send(crate::debugger::DebuggerEvent::FileLoaded {
                                    filename: filename_clone.clone(),
                                    content,
                                });
                            }
                        }
                    }) as Box<dyn FnMut(_)>);
                    file_reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
                    on_load.forget();
                    file_reader.read_as_text(&file).unwrap();
                }
            }
        }) as Box<dyn FnMut(_)>);
        html_input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
        on_change.forget();
        html_input.click();
    }
}

impl eframe::App for ShellcideApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll event queue
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                DebuggerEvent::Started { pid, regs, pc_breakpoints } => {
                    self.is_running = true;
                    self.is_stopped = false;
                    self.previous_regs = None;
                    self.breakpoint_originals = pc_breakpoints;
                    *self.shared_pid.lock().unwrap() = Some(pid);
                    let old_regs = self.regs;
                    self.regs = regs;
                    self.update_register_change_timestamps(old_regs, regs);
                    self.log("[Debugger] Child process spawned and running.");
                    self.refresh_memory();
                    self.refresh_disassembly_from_running_process();
                }
                DebuggerEvent::Stopped {
                    regs,
                    signal,
                    trap_addr,
                    status_message,
                    pc_breakpoints,
                } => {
                    self.is_running = true;
                    self.is_stopped = true;
                    self.previous_regs = Some(self.regs);
                    self.breakpoint_originals = pc_breakpoints;
                    let old_regs = self.regs;
                    self.regs = regs;
                    self.update_register_change_timestamps(old_regs, regs);
                    let at_addr =
                        trap_addr.map_or(String::new(), |addr| format!(" at 0x{:X}", addr));
                    self.log(&format!(
                        "[Debugger] Process stopped (Signal: {:?}){}. {}",
                        signal, at_addr, status_message
                    ));
                    self.refresh_memory();
                    self.refresh_disassembly_from_running_process();
                }
                DebuggerEvent::Terminated {
                    exit_code,
                    status_message,
                } => {
                    self.is_running = false;
                    self.is_stopped = false;
                    *self.shared_pid.lock().unwrap() = None;
                    self.log(&format!(
                        "[Debugger] Process terminated with status code {}: {}",
                        exit_code, status_message
                    ));
                    self.refresh_memory();
                }
                DebuggerEvent::Signaled {
                    signal,
                    status_message,
                } => {
                    self.is_running = false;
                    self.is_stopped = false;
                    *self.shared_pid.lock().unwrap() = None;
                    self.log(&format!(
                        "[Debugger] Process killed by signal: {:?}. {}",
                        signal, status_message
                    ));
                    self.refresh_memory();
                }
                DebuggerEvent::Stdout(data) => {
                    self.stdout_log.push_str(&data);
                }
                DebuggerEvent::Stderr(data) => {
                    self.stderr_log.push_str(&data);
                }
                DebuggerEvent::Error(err) => {
                    self.log(&format!("[Debugger Error] {}", err));
                }
                DebuggerEvent::FileLoaded { filename, content } => {
                    self.active_path = filename;
                    self.code_input = content;
                    self.log(&format!("[File] Loaded file: {}", self.active_path));
                }
            }
        }

        if self.is_running && self.is_stopped {
            let mut refreshed = false;
            if self.auto_follow_rsp {
                let rsp_aligned = self.get_centered_rsp();
                if self.memory_base_address != rsp_aligned {
                    self.jump_to_memory_address(rsp_aligned);
                    refreshed = true;
                }
            }
            if !refreshed {
                self.refresh_memory();
            }
        }

        crate::ui::header::render_header_panel(self, ctx);

        // Left Panel (Code Editor & Tabs at bottom)
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(200.0)
            .default_width(420.0)
            .show(ctx, |ui| {
                crate::ui::editor::render_editor_panel(self, ui);
                ui.separator();

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.left_bottom_tab,
                        LeftBottomTab::Syscalls,
                        "Syscalls",
                    );
                    ui.selectable_value(
                        &mut self.left_bottom_tab,
                        LeftBottomTab::StructPacker,
                        "Struct Packer",
                    );
                    ui.selectable_value(
                        &mut self.left_bottom_tab,
                        LeftBottomTab::Instructions,
                        "Instructions",
                    );
                });
                ui.separator();

                match self.left_bottom_tab {
                    LeftBottomTab::Syscalls => {
                        crate::ui::syscalls::render_syscalls_panel(self, ui);
                    }
                    LeftBottomTab::StructPacker => {
                        crate::ui::struct_packer::render_struct_packer_panel(self, ui);
                    }
                    LeftBottomTab::Instructions => {
                        crate::ui::instructions::render_instructions_panel(self, ui);
                    }
                }
            });

        // Right Panel (Registers & Memory)
        if self.is_native_debug() {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .min_width(200.0)
                .default_width(450.0)
                .show(ctx, |ui| {
                    egui::TopBottomPanel::bottom("right_memory_panel")
                        .resizable(true)
                        .default_height(350.0)
                        .min_height(20.0)
                        .max_height(1000.0)
                        .show_inside(ui, |ui| {
                            crate::ui::memory::render_memory_panel(self, ui);
                        });
                    crate::ui::registers::render_registers_panel(self, ui);
                });
        }

        // Center Panel (Controls, Disassembly, Logs Console)
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::bottom("console_panel")
                .resizable(true)
                .default_height(200.0)
                .min_height(20.0)
                .max_height(1000.0)
                .show_inside(ui, |ui| {
                    crate::ui::controls::render_console_panel(self, ui);
                });
            crate::ui::controls::render_controls_panel(self, ui);
        });

        // Continuous redraw when debugger is actively running in background
        if self.is_running && !self.is_stopped {
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
impl ShellcideApp {
    pub(crate) fn dummy(bytes: Vec<u8>) -> Self {
        let (cmd_tx, _) = flume::unbounded();
        let (event_tx, event_rx) = flume::unbounded();
        let shared_pid = Arc::new(Mutex::new(None));
        Self {
            code_input: String::new(),
            att_syntax: false,
            target_arch: TargetArch::X86_64,
            active_path: String::new(),
            cmd_tx,
            event_tx,
            event_rx,
            shared_pid,
            is_running: false,
            is_stopped: false,
            regs: CpuRegisters::default(),
            previous_regs: None,
            disassembly: Vec::new(),
            breakpoints: HashSet::new(),
            editor_breakpoints: HashSet::new(),
            stdout_log: String::new(),
            stderr_log: String::new(),
            console_log: String::new(),
            active_tab: ConsoleTab::Console,
            mem_base_input: String::new(),
            memory_base_address: 0,
            memory_data: vec![0; 256],
            editing_register: None,
            register_input: String::new(),
            editing_memory_byte: None,
            memory_byte_input: String::new(),
            compiled_bytes: bytes,
            syscall_search: String::new(),
            instructions_search: String::new(),
            bad_chars_input: "00".to_string(),
            bad_char_lines: HashSet::new(),
            reg_change_times: HashMap::new(),
            auto_follow_rsp: false,
            left_bottom_tab: LeftBottomTab::Syscalls,
            struct_packer: StructPackerState::default(),
            compression_type: crate::encoder::CompressionType::None,
            resolved_compression_marker: None,
            encryption_type: crate::encoder::EncryptionType::None,
            encryption_key: String::new(),
            encoding_type: crate::encoder::EncodingType::None,
            resolved_encoding_key: None,
            breakpoint_originals: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_app(bytes: Vec<u8>) -> ShellcideApp {
        ShellcideApp::dummy(bytes)
    }

    #[test]
    fn test_format_raw_hex() {
        let app = dummy_app(vec![0x90, 0xcc, 0x48, 0x31, 0xc0]);
        assert_eq!(app.format_raw_hex(&app.compiled_bytes), "90cc4831c0");
    }

    #[test]
    fn test_pipeline_with_rle_compression() {
        let mut app = dummy_app(vec![]);
        app.compression_type = crate::encoder::CompressionType::Rle;
        app.bad_chars_input = "00".to_string(); // avoid null bytes
        
        // 10 identical NOPs
        let payload = vec![0x90; 10];
        let pipeline_res = app.run_post_processing_pipeline(&payload);
        assert!(pipeline_res.is_ok());
        let final_bytes = pipeline_res.unwrap();
        
        // Must contain RLE stub bytes and compressed payload
        assert!(!final_bytes.is_empty());
        assert!(app.resolved_compression_marker.is_some());
        let marker = app.resolved_compression_marker.unwrap();
        assert_ne!(marker, 0x00);
        assert_ne!(marker, 0x90);
    }

    #[test]
    fn test_format_python() {
        let app_empty = dummy_app(vec![]);
        assert_eq!(app_empty.format_python(&app_empty.compiled_bytes), "shellcode = b\"\"");

        let app_short = dummy_app(vec![0x90, 0xcc]);
        assert_eq!(
            app_short.format_python(&app_short.compiled_bytes),
            "shellcode = b\"\"\nshellcode += b\"\\x90\\xcc\""
        );

        let app_long = dummy_app((0..20).collect());
        let expected = "shellcode = b\"\"\nshellcode += b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x0a\\x0b\\x0c\\x0d\\x0e\\x0f\"\nshellcode += b\"\\x10\\x11\\x12\\x13\"";
        assert_eq!(app_long.format_python(&app_long.compiled_bytes), expected);
    }

    #[test]
    fn test_format_c() {
        let app_empty = dummy_app(vec![]);
        assert_eq!(app_empty.format_c(&app_empty.compiled_bytes), "unsigned char shellcode[] = \"\";");

        let app_short = dummy_app(vec![0x90, 0xcc]);
        assert_eq!(
            app_short.format_c(&app_short.compiled_bytes),
            "unsigned char shellcode[] = \n\"\\x90\\xcc\";"
        );

        let app_long = dummy_app((0..20).collect());
        let expected = "unsigned char shellcode[] = \n\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x0a\\x0b\\x0c\\x0d\\x0e\\x0f\"\n\"\\x10\\x11\\x12\\x13\";";
        assert_eq!(app_long.format_c(&app_long.compiled_bytes), expected);
    }

    #[test]
    fn test_format_rust() {
        let app_empty = dummy_app(vec![]);
        assert_eq!(app_empty.format_rust(&app_empty.compiled_bytes), "const SHELLCODE: &[u8] = &[];");

        let app_short = dummy_app(vec![0x90, 0xcc]);
        assert_eq!(
            app_short.format_rust(&app_short.compiled_bytes),
            "const SHELLCODE: &[u8] = &[\n    0x90, 0xcc, \n];"
        );

        let app_long = dummy_app((0..15).collect());
        let expected = "const SHELLCODE: &[u8] = &[\n    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, \n    0x0c, 0x0d, 0x0e, \n];";
        assert_eq!(app_long.format_rust(&app_long.compiled_bytes), expected);
    }

    #[test]
    fn test_extract_arg_name() {
        use crate::ui::syscalls::extract_arg_name;
        assert_eq!(extract_arg_name("unsigned int fd"), "fd");
        assert_eq!(extract_arg_name("char *buf"), "buf");
        assert_eq!(extract_arg_name("const char *filename"), "filename");
        assert_eq!(extract_arg_name("int"), "int");
        assert_eq!(extract_arg_name("unsigned"), "unsigned");
        assert_eq!(extract_arg_name("const struct sockaddr *"), "sockaddr");
        assert_eq!(extract_arg_name("unsigned long"), "long");
    }

    #[test]
    fn test_do_assemble_bad_chars_warning() {
        let mut app = dummy_app(vec![]);
        app.code_input = "nop".to_string();
        app.bad_chars_input = "90".to_string();
        app.do_assemble();

        assert!(app.console_log.contains("compiled successfully"));
        assert!(app
            .console_log
            .contains("[⚠️ WARNING] Found 1 bad character(s)"));
        assert!(app.console_log.contains("0x90 at 0x10000000"));
        assert!(app.bad_char_lines.contains(&0));
    }

    #[test]
    fn test_memory_jump_rsp_rbp() {
        let mut app = dummy_app(vec![]);
        app.regs.rsp = 0x7fffffffe000;
        app.regs.rbp = 0x7fffffffe010;

        // Simulate Jump to RSP button click logic
        let rsp_aligned = app.get_centered_rsp();
        app.jump_to_memory_address(rsp_aligned);
        assert_eq!(app.memory_base_address, 0x7fffffffdf80);
        assert_eq!(app.mem_base_input, "0x7FFFFFFFDF80");

        // Simulate Jump to RBP button click logic
        app.jump_to_memory_address(app.regs.rbp as usize);
        assert_eq!(app.memory_base_address, 0x7fffffffe010);
        assert_eq!(app.mem_base_input, "0x7FFFFFFFE010");
    }

    #[test]
    fn test_auto_follow_rsp() {
        let mut app = dummy_app(vec![]);
        app.is_running = true;
        app.is_stopped = true;
        app.auto_follow_rsp = true;
        app.regs.rsp = 0x7fffffffe008; // unaligned RSP

        let rsp_aligned = app.get_centered_rsp();
        if app.memory_base_address != rsp_aligned {
            app.jump_to_memory_address(rsp_aligned);
        }

        assert_eq!(app.memory_base_address, 0x7fffffffdf80);
        assert_eq!(app.mem_base_input, "0x7FFFFFFFDF80");
    }
}
