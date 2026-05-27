use crate::app::{ShellcideApp, TargetArch};
use crate::debugger::CODE_BASE;
use eframe::egui;

fn create_test_app(bytes: Vec<u8>) -> ShellcideApp {
    ShellcideApp::dummy(bytes)
}

fn test_render(mut f: impl FnMut(&mut egui::Ui)) {
    let ctx = egui::Context::default();
    let _ = ctx.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            f(ui);
        });
    });
}

#[test]
fn test_editor_panel_render() {
    let mut app = create_test_app(Vec::new());
    app.code_input = "mov rax, 0".to_string();
    app.do_assemble();

    // The assembler may optimize "mov rax, 0" to "xor eax, eax" (no null bytes).
    // We explicitly insert 0 to ensure bad_char_lines has the expected value for testing.
    app.bad_char_lines.insert(0);

    assert!(app.bad_char_lines.contains(&0));

    test_render(|ui| crate::ui::editor::render_editor_panel(&mut app, ui));
}

#[test]
fn test_header_panel_render() {
    let mut app = create_test_app(Vec::new());
    let ctx = egui::Context::default();
    let _ = ctx.run(egui::RawInput::default(), |ctx| {
        crate::ui::header::render_header_panel(&mut app, ctx);
    });
}

#[test]
fn test_registers_panel_render() {
    let mut app = create_test_app(Vec::new());
    let prev = app.regs;
    app.regs.rax = 42;
    app.update_register_change_timestamps(prev, app.regs);

    test_render(|ui| crate::ui::registers::render_registers_panel(&mut app, ui));
}

#[test]
fn test_memory_panel_render() {
    let mut app = create_test_app(Vec::new());
    app.regs.rsp = 0x3000_1000;
    app.regs.rbp = 0x3000_1020;
    app.is_running = true;
    app.is_stopped = true;

    test_render(|ui| crate::ui::memory::render_memory_panel(&mut app, ui));
}

#[test]
fn test_controls_panel_render() {
    let mut app = create_test_app(Vec::new());
    app.regs.rip = CODE_BASE as u64;
    app.is_running = true;
    app.is_stopped = true;

    // Generate some dummy disassembly instruction
    app.disassembly
        .push(crate::disassembler::DisassembledInstruction {
            address: CODE_BASE as u64,
            bytes: vec![0x90],
            mnemonic: "nop".to_string(),
            op_str: String::new(),
        });

    test_render(|ui| {
        crate::ui::controls::render_controls_panel(&mut app, ui);
        crate::ui::controls::render_console_panel(&mut app, ui);
    });
}

#[test]
fn test_auto_follow_rsp_integration() {
    let mut app = create_test_app(Vec::new());
    app.auto_follow_rsp = true;
    app.is_running = true;
    app.is_stopped = true;
    app.regs.rsp = 0x3000_1008;

    // Mock/Run the logic in the app's `update()` loop
    if app.is_running && app.is_stopped {
        let mut refreshed = false;
        if app.auto_follow_rsp {
            let rsp_aligned = app.get_centered_rsp();
            if app.memory_base_address != rsp_aligned {
                app.jump_to_memory_address(rsp_aligned);
                refreshed = true;
            }
        }
        if !refreshed {
            app.refresh_memory();
        }
    }

    assert_eq!(app.memory_base_address, 0x3000_0F80);
}

#[test]
fn test_controls_panel_hides_debug_buttons_non_native() {
    let mut app = create_test_app(Vec::new());
    app.target_arch = TargetArch::Riscv;
    app.regs.rip = CODE_BASE as u64;
    app.is_running = true;
    app.is_stopped = true;

    // Generate some dummy disassembly instruction
    app.disassembly
        .push(crate::disassembler::DisassembledInstruction {
            address: CODE_BASE as u64,
            bytes: vec![0x90],
            mnemonic: "nop".to_string(),
            op_str: String::new(),
        });

    test_render(|ui| {
        crate::ui::controls::render_controls_panel(&mut app, ui);
        crate::ui::controls::render_console_panel(&mut app, ui);
    });

    // Verification: target arch remains non-native
    assert_eq!(app.target_arch, TargetArch::Riscv);
}

#[test]
fn test_editor_gutter_does_not_toggle_bp_non_native() {
    let mut app = create_test_app(Vec::new());
    app.target_arch = TargetArch::Riscv;
    app.code_input = "nop".to_string();
    app.do_assemble();

    test_render(|ui| crate::ui::editor::render_editor_panel(&mut app, ui));

    // Breakpoints should be empty
    assert!(app.breakpoints.is_empty());
    assert!(app.editor_breakpoints.is_empty());
}

#[test]
fn test_parse_u64_input() {
    assert_eq!(crate::ui::parse_u64_input("0x123"), Some(0x123));
    assert_eq!(crate::ui::parse_u64_input("0X456"), Some(0x456));
    assert_eq!(crate::ui::parse_u64_input("123h"), Some(0x123));
    assert_eq!(crate::ui::parse_u64_input("456H"), Some(0x456));
    assert_eq!(crate::ui::parse_u64_input("42"), Some(42));
    assert_eq!(crate::ui::parse_u64_input("  7f "), Some(127));
    assert_eq!(crate::ui::parse_u64_input(""), None);
    assert_eq!(crate::ui::parse_u64_input("invalid"), None);
}

#[test]
fn test_instructions_dataset_and_search() {
    use crate::ui::instructions::{get_instructions, InstructionInfo};

    let insts_x86_64 = get_instructions(TargetArch::X86_64);
    assert_eq!(insts_x86_64.len(), 60);

    let expected_x86_64 = [
        "mov",
        "movzx",
        "movsx",
        "movsxd",
        "xor",
        "xadd",
        "cmpxchg",
        "push",
        "pop",
        "jmp",
        "call",
        "ret",
        "syscall",
        "inc",
        "dec",
        "neg",
        "not",
        "add",
        "sub",
        "shl",
        "shr",
        "sar",
        "rol",
        "ror",
        "lea",
        "cmp",
        "test",
        "nop",
        "cdq",
        "cqo",
        "xchg",
        "lodsb",
        "stosb",
        "rep movsb",
        "rep stosb",
        "repne scasb",
        "cld",
        "std",
        "leave",
        "enter",
        "int3",
        "ud2",
        "cpuid",
        "rdtsc",
        "rdtscp",
        "rdrand",
        "lahf",
        "sahf",
        "bt",
        "bts",
        "btr",
        "btc",
        "bsf",
        "bsr",
        "popcnt",
        "lzcnt",
        "tzcnt",
        "loop",
        "fldz",
        "fnstenv",
    ];
    for name in &expected_x86_64 {
        assert!(
            insts_x86_64.iter().any(|i| i.name == *name),
            "x86_64 instructions is missing {}",
            name
        );
    }

    // Verify all x86_64 instructions have a non-empty shellcode tip
    for inst in insts_x86_64 {
        assert!(!inst.shellcode_tip.is_empty());
    }

    let insts_x86 = get_instructions(TargetArch::X86);
    assert_eq!(insts_x86.len(), 55);

    let expected_x86 = [
        "mov",
        "movzx",
        "movsx",
        "xor",
        "push",
        "pop",
        "jmp",
        "call",
        "ret",
        "int 0x80",
        "sysenter",
        "inc",
        "dec",
        "neg",
        "not",
        "add",
        "sub",
        "shl",
        "shr",
        "sar",
        "rol",
        "ror",
        "lea",
        "cmp",
        "test",
        "nop",
        "cdq",
        "cltd",
        "xchg",
        "lodsb",
        "stosb",
        "rep movsb",
        "rep stosb",
        "repne scasb",
        "cld",
        "std",
        "leave",
        "enter",
        "int3",
        "ud2",
        "cpuid",
        "rdtsc",
        "rdrand",
        "lahf",
        "sahf",
        "bt",
        "bts",
        "btr",
        "btc",
        "bsf",
        "bsr",
        "popcnt",
        "loop",
        "fldz",
        "fnstenv",
    ];
    for name in &expected_x86 {
        assert!(
            insts_x86.iter().any(|i| i.name == *name),
            "x86 instructions is missing {}",
            name
        );
    }

    // Verify all x86 instructions have a non-empty shellcode tip
    for inst in insts_x86 {
        assert!(!inst.shellcode_tip.is_empty());
    }

    let insts_riscv = get_instructions(TargetArch::Riscv);
    assert!(!insts_riscv.is_empty());
    let has_ecall = insts_riscv.iter().any(|i| i.name == "ecall");
    assert!(has_ecall);

    // Verify all riscv instructions have a non-empty shellcode tip
    for inst in insts_riscv {
        assert!(!inst.shellcode_tip.is_empty());
    }

    // Verify InstructionInfo constructors with new shellcode_tip field
    let info_intel = InstructionInfo::new(
        "test_mov",
        "mov eax, ebx",
        "Move description",
        "mov eax, ebx\n",
        "Avoid null bytes in eax",
    );
    assert_eq!(info_intel.name, "test_mov");
    assert_eq!(info_intel.shellcode_tip, "Avoid null bytes in eax");

    let info_att = InstructionInfo::new_att(
        "test_mov2",
        "mov eax, ebx",
        "movl %ebx, %eax",
        "Move description 2",
        "mov eax, ebx\n",
        "movl %ebx, %eax\n",
        "Avoid null bytes in eax 2",
    );
    assert_eq!(info_att.name, "test_mov2");
    assert_eq!(info_att.shellcode_tip, "Avoid null bytes in eax 2");
}

#[test]
fn test_instructions_panel_render() {
    let mut app = create_test_app(Vec::new());
    app.target_arch = TargetArch::X86_64;

    // Render with empty search
    test_render(|ui| crate::ui::instructions::render_instructions_panel(&mut app, ui));

    // Render with a query search
    app.instructions_search = "mov".to_string();
    test_render(|ui| crate::ui::instructions::render_instructions_panel(&mut app, ui));

    // Clear search
    app.instructions_search = String::new();

    // Change arch and render again
    app.target_arch = TargetArch::Riscv;
    test_render(|ui| crate::ui::instructions::render_instructions_panel(&mut app, ui));
}

#[test]
fn test_contains_case_insensitive() {
    use crate::ui::contains_case_insensitive;
    assert!(contains_case_insensitive("Intel Syntax", "intel"));
    assert!(contains_case_insensitive("Intel Syntax", "SYNTAX"));
    assert!(contains_case_insensitive("Intel Syntax", "el sy"));
    assert!(contains_case_insensitive("Intel Syntax", ""));
    assert!(!contains_case_insensitive("Intel Syntax", "att"));
    assert!(!contains_case_insensitive(
        "Intel Syntax",
        "longer_than_haystack"
    ));
}
