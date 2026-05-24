use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};
use crossbeam_channel::{Receiver, Sender};
use eframe::egui;
use nix::unistd::Pid;

use crate::debugger::{CpuRegisters, DebuggerCommand, DebuggerEvent, CODE_BASE, DATA_BASE};
use crate::assembler::assemble;
use crate::disassembler::{disassemble_code, DisassembledInstruction};

pub struct ShellcideApp {
    // Code input and settings
    pub(crate) code_input: String,
    pub(crate) att_syntax: bool,
    pub(crate) active_path: String,

    // Thread communication
    pub(crate) cmd_tx: Sender<DebuggerCommand>,
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
    pub(crate) bad_chars_input: String,
}

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
    Shellcode,
}

impl ShellcideApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        cmd_tx: Sender<DebuggerCommand>,
        event_rx: Receiver<DebuggerEvent>,
        shared_pid: Arc<Mutex<Option<Pid>>>,
    ) -> Self {
        crate::ui::theme::apply_cyber_cyan_theme(&_cc.egui_ctx);

        let default_code = crate::assembler::DEMO_CODE;

        Self {
            code_input: default_code.to_string(),
            att_syntax: false,
            active_path: "demo.s".to_string(),
            cmd_tx,
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
            bad_chars_input: String::new(),
        }
    }

    pub(crate) fn log(&mut self, msg: &str) {
        self.console_log.push_str(msg);
        self.console_log.push('\n');
    }

    pub(crate) fn get_line_to_inst_mapping(&self) -> HashMap<usize, usize> {
        let mut line_to_inst_idx = HashMap::new();
        let mut inst_idx = 0;
        for (line_idx, line) in self.code_input.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                continue;
            }
            if trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }
            line_to_inst_idx.insert(line_idx, inst_idx);
            inst_idx += 1;
        }
        line_to_inst_idx
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
        let line_to_inst_idx = self.get_line_to_inst_mapping();
        for &addr in &self.breakpoints {
            if let Some(inst_idx) = self.disassembly.iter().position(|inst| inst.address as usize == addr) {
                if let Some(&line_idx) = line_to_inst_idx.iter().find(|&(_, &idx)| idx == inst_idx).map(|(l, _)| l) {
                    self.editor_breakpoints.insert(line_idx);
                }
            }
        }
    }

    pub(crate) fn do_assemble(&mut self) {
        self.log("[+] Assembling shellcode...");
        match assemble(&self.code_input, CODE_BASE as u64, self.att_syntax) {
            Ok(bytes) => {
                self.log(&format!("[✓] Shellcode compiled successfully. Size: {} bytes.", bytes.len()));
                
                let bad_chars = crate::assembler::parse_bad_characters(&self.bad_chars_input);
                if !bad_chars.is_empty() {
                    let mut found_bad_bytes = Vec::new();
                    for (i, &b) in bytes.iter().enumerate() {
                        if bad_chars.contains(&b) {
                            let addr = CODE_BASE + i;
                            found_bad_bytes.push(format!("0x{:02X} at 0x{:08X}", b, addr));
                        }
                    }
                    if !found_bad_bytes.is_empty() {
                        self.log(&format!(
                            "[⚠️ WARNING] Found {} bad character(s) in compiled code:\n    {}",
                            found_bad_bytes.len(),
                            found_bad_bytes.join(", ")
                        ));
                    }
                }

                self.compiled_bytes = bytes.clone();
                self.disassembly = disassemble_code(&bytes, CODE_BASE as u64, self.att_syntax);
                
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

    pub(crate) fn format_raw_hex(&self) -> String {
        self.compiled_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    }

    pub(crate) fn format_python(&self) -> String {
        if self.compiled_bytes.is_empty() {
            return "shellcode = b\"\"".to_string();
        }
        let mut out = "shellcode = b\"\"\nshellcode += b\"".to_string();
        for (i, &b) in self.compiled_bytes.iter().enumerate() {
            if i > 0 && i % 16 == 0 {
                out.push_str("\"\nshellcode += b\"");
            }
            out.push_str(&format!("\\x{:02x}", b));
        }
        out.push('"');
        out
    }

    pub(crate) fn format_c(&self) -> String {
        if self.compiled_bytes.is_empty() {
            return "unsigned char shellcode[] = \"\";".to_string();
        }
        let mut out = "unsigned char shellcode[] = \n\"".to_string();
        for (i, &b) in self.compiled_bytes.iter().enumerate() {
            if i > 0 && i % 16 == 0 {
                out.push_str("\"\n\"");
            }
            out.push_str(&format!("\\x{:02x}", b));
        }
        out.push_str("\";");
        out
    }

    pub(crate) fn format_rust(&self) -> String {
        if self.compiled_bytes.is_empty() {
            return "const SHELLCODE: &[u8] = &[];".to_string();
        }
        let mut out = "const SHELLCODE: &[u8] = &[\n    ".to_string();
        for (i, &b) in self.compiled_bytes.iter().enumerate() {
            if i > 0 && i % 12 == 0 {
                out.push_str("\n    ");
            }
            out.push_str(&format!("0x{:02x}, ", b));
        }
        out.push_str("\n];");
        out
    }
}

impl eframe::App for ShellcideApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll event queue
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                DebuggerEvent::Started { regs } => {
                    self.is_running = true;
                    self.is_stopped = false;
                    self.previous_regs = None;
                    self.regs = regs;
                    self.log("[Debugger] Child process spawned and running.");
                    self.refresh_memory();
                }
                DebuggerEvent::Stopped { regs, signal, trap_addr, status_message } => {
                    self.is_running = true;
                    self.is_stopped = true;
                    self.previous_regs = Some(self.regs);
                    self.regs = regs;
                    let at_addr = trap_addr.map_or(String::new(), |addr| format!(" at 0x{:X}", addr));
                    self.log(&format!("[Debugger] Process stopped (Signal: {:?}){}. {}", signal, at_addr, status_message));
                    self.refresh_memory();
                }
                DebuggerEvent::Terminated { exit_code, status_message } => {
                    self.is_running = false;
                    self.is_stopped = false;
                    self.log(&format!("[Debugger] Process terminated with status code {}: {}", exit_code, status_message));
                    self.refresh_memory();
                }
                DebuggerEvent::Signaled { signal, status_message } => {
                    self.is_running = false;
                    self.is_stopped = false;
                    self.log(&format!("[Debugger] Process killed by signal: {:?}. {}", signal, status_message));
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
            }
        }

        crate::ui::header::render_header_panel(self, ctx);

        // Left Panel (Code Editor & Syscall Help)
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(200.0)
            .default_width(420.0)
            .show(ctx, |ui| {
                crate::ui::editor::render_editor_panel(self, ui);
                ui.separator();
                crate::ui::syscalls::render_syscalls_panel(self, ui);
            });

        // Right Panel (Registers & Memory)
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .min_width(200.0)
            .default_width(450.0)
            .show(ctx, |ui| {
                crate::ui::registers::render_registers_panel(self, ui);
                ui.separator();
                crate::ui::memory::render_memory_panel(self, ui);
            });

        // Center Panel (Controls, Disassembly, Logs Console)
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::controls::render_controls_panel(self, ui);
        });

        // Continuous redraw when debugger is actively running in background
        if self.is_running && !self.is_stopped {
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
fn extract_arg_name(arg_type: &str) -> &str {
    let trimmed = arg_type.trim();
    if trimmed.is_empty() {
        return "";
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return "";
    }
    let last = parts[parts.len() - 1];
    let mut name = last;
    while name.starts_with('*') {
        name = &name[1..];
    }
    if name.is_empty() {
        for part in parts.iter().rev().skip(1) {
            let mut p = *part;
            while p.starts_with('*') {
                p = &p[1..];
            }
            if !p.is_empty() && p != "const" && p != "struct" {
                return p;
            }
        }
        return arg_type;
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_app(bytes: Vec<u8>) -> ShellcideApp {
        let (cmd_tx, _) = crossbeam_channel::unbounded();
        let (_, event_rx) = crossbeam_channel::unbounded();
        let shared_pid = Arc::new(Mutex::new(None));
        
        ShellcideApp {
            code_input: String::new(),
            att_syntax: false,
            active_path: String::new(),
            cmd_tx,
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
            memory_data: Vec::new(),
            editing_register: None,
            register_input: String::new(),
            editing_memory_byte: None,
            memory_byte_input: String::new(),
            compiled_bytes: bytes,
            syscall_search: String::new(),
            bad_chars_input: String::new(),
        }
    }

    #[test]
    fn test_format_raw_hex() {
        let app = dummy_app(vec![0x90, 0xcc, 0x48, 0x31, 0xc0]);
        assert_eq!(app.format_raw_hex(), "90cc4831c0");
    }

    #[test]
    fn test_format_python() {
        let app_empty = dummy_app(vec![]);
        assert_eq!(app_empty.format_python(), "shellcode = b\"\"");

        let app_short = dummy_app(vec![0x90, 0xcc]);
        assert_eq!(app_short.format_python(), "shellcode = b\"\"\nshellcode += b\"\\x90\\xcc\"");

        let app_long = dummy_app((0..20).collect());
        let expected = "shellcode = b\"\"\nshellcode += b\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x0a\\x0b\\x0c\\x0d\\x0e\\x0f\"\nshellcode += b\"\\x10\\x11\\x12\\x13\"";
        assert_eq!(app_long.format_python(), expected);
    }

    #[test]
    fn test_format_c() {
        let app_empty = dummy_app(vec![]);
        assert_eq!(app_empty.format_c(), "unsigned char shellcode[] = \"\";");

        let app_short = dummy_app(vec![0x90, 0xcc]);
        assert_eq!(app_short.format_c(), "unsigned char shellcode[] = \n\"\\x90\\xcc\";");

        let app_long = dummy_app((0..20).collect());
        let expected = "unsigned char shellcode[] = \n\"\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x0a\\x0b\\x0c\\x0d\\x0e\\x0f\"\n\"\\x10\\x11\\x12\\x13\";";
        assert_eq!(app_long.format_c(), expected);
    }

    #[test]
    fn test_format_rust() {
        let app_empty = dummy_app(vec![]);
        assert_eq!(app_empty.format_rust(), "const SHELLCODE: &[u8] = &[];");

        let app_short = dummy_app(vec![0x90, 0xcc]);
        assert_eq!(app_short.format_rust(), "const SHELLCODE: &[u8] = &[\n    0x90, 0xcc, \n];");

        let app_long = dummy_app((0..15).collect());
        let expected = "const SHELLCODE: &[u8] = &[\n    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, \n    0x0c, 0x0d, 0x0e, \n];";
        assert_eq!(app_long.format_rust(), expected);
    }

    #[test]
    fn test_extract_arg_name() {
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
        assert!(app.console_log.contains("[⚠️ WARNING] Found 1 bad character(s)"));
        assert!(app.console_log.contains("0x90 at 0x10000000"));
    }
}


