use eframe::egui::{self, Color32};
use crate::app::{ShellcideApp, ConsoleTab};
use crate::debugger::DebuggerCommand;

pub fn render_controls_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("Debugger Controls");
    ui.separator();

    // Command buttons
    ui.horizontal(|ui| {
        // Assemble Code
        let compile_btn = ui.add(
            egui::Button::new(egui::RichText::new("⚙ Assemble").strong().color(Color32::BLACK))
                .fill(Color32::from_rgb(116, 185, 255))
        );
        if compile_btn.clicked() {
            app.do_assemble();
        }

        ui.separator();

        // Run/Start
        let run_enabled = !app.disassembly.is_empty();
        let start_btn = ui.add_enabled(
            run_enabled && !app.is_running,
            egui::Button::new(egui::RichText::new("▶ Run / Debug").strong().color(Color32::BLACK))
                .fill(Color32::from_rgb(85, 239, 196))
        );
        if start_btn.clicked() {
            app.log("[+] Spawning tracee process in background...");
            let patches = Vec::new(); // Initial memory patches (if any)
            let bps = app.breakpoints.iter().cloned().collect();
            let file_bytes = app.compiled_bytes.clone();
            app.cmd_tx.send(DebuggerCommand::Start {
                code: file_bytes,
                initial_regs: app.regs,
                memory_patches: patches,
                breakpoints: bps,
            }).unwrap();
        }

        // Step Into
        let step_btn = ui.add_enabled(
            app.is_running && app.is_stopped,
            egui::Button::new(egui::RichText::new("➡ Step Into").strong().color(Color32::BLACK))
                .fill(Color32::from_rgb(253, 121, 168))
        );
        if step_btn.clicked() {
            app.cmd_tx.send(DebuggerCommand::Step).unwrap();
        }

        // Continue
        let cont_btn = ui.add_enabled(
            app.is_running && app.is_stopped,
            egui::Button::new(egui::RichText::new("⏩ Continue").strong().color(Color32::BLACK))
                .fill(Color32::from_rgb(0, 206, 201))
        );
        if cont_btn.clicked() {
            app.cmd_tx.send(DebuggerCommand::Continue).unwrap();
        }

        // Pause
        let pause_btn = ui.add_enabled(
            app.is_running && !app.is_stopped,
            egui::Button::new(egui::RichText::new("⏸ Pause").strong().color(Color32::BLACK))
                .fill(Color32::from_rgb(254, 202, 87))
        );
        if pause_btn.clicked() {
            app.cmd_tx.send(DebuggerCommand::Pause).unwrap();
        }

        // Terminate
        let stop_btn = ui.add_enabled(
            app.is_running,
            egui::Button::new(egui::RichText::new("⏹ Stop").strong().color(Color32::BLACK))
                .fill(Color32::from_rgb(255, 118, 117))
        );
        if stop_btn.clicked() {
            app.cmd_tx.send(DebuggerCommand::Terminate).unwrap();
        }
    });

    ui.separator();
    ui.heading("Shellcode Disassembly");
    ui.separator();

    // Disassembly Instruction View
    let disasm_height = ui.available_height() - 250.0;
    egui::ScrollArea::vertical().id_salt("disasm_scroll").max_height(disasm_height).show(ui, |ui| {
        if app.disassembly.is_empty() {
            ui.colored_label(Color32::GRAY, "(Assemble shellcode to view disassembly)");
        } else {
            egui::Grid::new("disasm_grid").num_columns(6).spacing([8.0, 4.0]).show(ui, |ui| {
                for inst in app.disassembly.clone() {
                    let addr = inst.address as usize;
                    let is_current = app.is_running && app.regs.rip == inst.address;
                    let has_bp = app.breakpoints.contains(&addr);

                    // 1. Breakpoint margin toggle button
                    let bp_text = if has_bp { "🔴" } else { "  " };
                    let bp_btn = ui.add(
                        egui::Button::new(egui::RichText::new(bp_text).monospace())
                            .frame(false)
                    );
                    if bp_btn.clicked() {
                        if has_bp {
                            app.breakpoints.remove(&addr);
                            app.cmd_tx.send(DebuggerCommand::ToggleBreakpoint(addr, false)).unwrap();
                            app.log(&format!("[Breakpoint] Removed at 0x{:08X}", addr));
                        } else {
                            app.breakpoints.insert(addr);
                            app.cmd_tx.send(DebuggerCommand::ToggleBreakpoint(addr, true)).unwrap();
                            app.log(&format!("[Breakpoint] Added at 0x{:08X}", addr));
                        }
                        app.sync_code_from_breakpoints();
                    }

                    // 2. Active RIP indicator arrow
                    let rip_indicator = if is_current { "👉" } else { "  " };
                    ui.label(egui::RichText::new(rip_indicator).monospace().strong().color(Color32::from_rgb(0, 206, 201)));

                    // 3. Instruction address
                    let addr_color = if is_current { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY };
                    ui.label(egui::RichText::new(format!("0x{:08X}:", addr)).monospace().color(addr_color));

                    // 4. Hex machine bytes
                    let bytes_str = inst.bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");
                    let bytes_color = if is_current { Color32::from_rgb(116, 185, 255) } else { Color32::from_rgb(99, 110, 114) };
                    ui.label(egui::RichText::new(bytes_str).monospace().color(bytes_color));

                    // 5. Mnemonic & Opcode operands
                    let text_color = if is_current { Color32::from_rgb(0, 206, 201) } else { Color32::WHITE };
                    ui.label(
                        egui::RichText::new(&inst.mnemonic)
                            .monospace()
                            .strong()
                            .color(text_color)
                    );
                    ui.label(
                        egui::RichText::new(&inst.op_str)
                            .monospace()
                            .color(text_color)
                    );
                    ui.end_row();
                }
            });
        }
    });

    ui.separator();

    // Logs Console with Tab layout
    ui.horizontal(|ui| {
        if ui.selectable_label(app.active_tab == ConsoleTab::Console, "Status Console").clicked() {
            app.active_tab = ConsoleTab::Console;
        }
        if ui.selectable_label(app.active_tab == ConsoleTab::Stdout, "Stdout").clicked() {
            app.active_tab = ConsoleTab::Stdout;
        }
        if ui.selectable_label(app.active_tab == ConsoleTab::Stderr, "Stderr").clicked() {
            app.active_tab = ConsoleTab::Stderr;
        }
        if ui.selectable_label(app.active_tab == ConsoleTab::Shellcode, "Shellcode Output").clicked() {
            app.active_tab = ConsoleTab::Shellcode;
        }
    });

    ui.separator();

    // Log Console text display
    if app.active_tab == ConsoleTab::Shellcode {
        egui::ScrollArea::vertical().id_salt("shellcode_scroll").max_height(200.0).show(ui, |ui| {
            if app.compiled_bytes.is_empty() {
                ui.colored_label(Color32::GRAY, "(Assemble shellcode to view formatted outputs)");
            } else {
                let formats = [
                    ("Raw Hex String", app.format_raw_hex()),
                    ("Python Variable", app.format_python()),
                    ("C Variable", app.format_c()),
                    ("Rust Variable", app.format_rust()),
                ];

                for (label, val) in formats {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(label).strong().color(Color32::from_rgb(0, 206, 201)));
                        if ui.button("📋 Copy").clicked() {
                            ui.ctx().copy_text(val.clone());
                        }
                    });
                    let mut temp_val = val.clone();
                    ui.add(
                        egui::TextEdit::multiline(&mut temp_val)
                            .font(egui::FontId::monospace(12.0))
                            .desired_width(f32::INFINITY)
                            .desired_rows((val.lines().count()).clamp(2, 6))
                            .interactive(true)
                    );
                    ui.add_space(8.0);
                }
            }
        });
    } else {
        let active_log = match app.active_tab {
            ConsoleTab::Console => &app.console_log,
            ConsoleTab::Stdout => &app.stdout_log,
            ConsoleTab::Stderr => &app.stderr_log,
            ConsoleTab::Shellcode => unreachable!(),
        };

        egui::ScrollArea::vertical().id_salt("console_scroll").max_height(200.0).show(ui, |ui| {
            let displayed_log = if active_log.is_empty() {
                "(empty)"
            } else {
                active_log
            };
            ui.add(
                egui::TextEdit::multiline(&mut displayed_log.to_string())
                    .font(egui::FontId::monospace(12.0))
                    .desired_width(f32::INFINITY)
                    .desired_rows(8)
                    .interactive(false)
            );
        });
    }
}
