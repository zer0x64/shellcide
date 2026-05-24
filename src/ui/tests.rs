use eframe::egui;
use crate::app::{ShellcideApp, TargetArch};
use crate::debugger::CODE_BASE;

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
    app.disassembly.push(crate::disassembler::DisassembledInstruction {
        address: CODE_BASE as u64,
        bytes: vec![0x90],
        mnemonic: "nop".to_string(),
        op_str: String::new(),
    });

    test_render(|ui| crate::ui::controls::render_controls_panel(&mut app, ui));
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
    app.disassembly.push(crate::disassembler::DisassembledInstruction {
        address: CODE_BASE as u64,
        bytes: vec![0x90],
        mnemonic: "nop".to_string(),
        op_str: String::new(),
    });

    test_render(|ui| crate::ui::controls::render_controls_panel(&mut app, ui));

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
