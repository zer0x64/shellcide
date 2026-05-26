use crate::app::{ConsoleTab, ShellcideApp, TargetArch};
use crate::debugger::DebuggerCommand;
use crate::ui::theme::{
    BRIGHT_RED, CYBER_CYAN, ICE_BLUE, MUSTARD_YELLOW, NEON_PINK, SLATE_GRAY, TOXIC_GREEN,
};
use eframe::egui::{self, Color32};

fn highlight_if(mut text: egui::RichText, cond: bool) -> egui::RichText {
    if cond {
        text = text.background_color(Color32::from_rgba_unmultiplied(
            CYBER_CYAN.r(),
            CYBER_CYAN.g(),
            CYBER_CYAN.b(),
            30,
        ));
    }
    text
}

fn dbg_button(ui: &mut egui::Ui, enabled: bool, text: &str, fill: Color32) -> egui::Response {
    ui.add_enabled(
        enabled,
        egui::Button::new(egui::RichText::new(text).strong().color(Color32::BLACK)).fill(fill),
    )
}

pub fn render_controls_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("Debugger Controls");
    ui.separator();

    // Command buttons
    ui.horizontal(|ui| {
        // Assemble Code
        let compile_btn = dbg_button(ui, true, "⚙ Assemble", ICE_BLUE);
        if compile_btn.clicked() {
            app.do_assemble();
        }

        if !cfg!(target_arch = "wasm32") && app.target_arch == TargetArch::X86_64 {
            ui.separator();

            // Run/Start
            let run_enabled = !app.disassembly.is_empty();
            let start_btn = dbg_button(
                ui,
                run_enabled && !app.is_running,
                "▶ Run / Debug",
                TOXIC_GREEN,
            );
            if start_btn.clicked() {
                app.log("[+] Spawning tracee process in background...");
                let patches = Vec::new(); // Initial memory patches (if any)
                let bps = app.breakpoints.iter().cloned().collect();
                let file_bytes = app.compiled_bytes.clone();
                app.cmd_tx
                    .send(DebuggerCommand::Start {
                        code: file_bytes,
                        initial_regs: app.regs,
                        memory_patches: patches,
                        breakpoints: bps,
                    })
                    .unwrap();
            }

            // Step Into
            let step_btn = dbg_button(
                ui,
                app.is_running && app.is_stopped,
                "➡ Step Into",
                NEON_PINK,
            );
            if step_btn.clicked() {
                app.cmd_tx.send(DebuggerCommand::Step).unwrap();
            }

            // Continue
            let cont_btn = dbg_button(
                ui,
                app.is_running && app.is_stopped,
                "⏩ Continue",
                CYBER_CYAN,
            );
            if cont_btn.clicked() {
                app.cmd_tx.send(DebuggerCommand::Continue).unwrap();
            }

            // Pause
            let pause_btn = dbg_button(
                ui,
                app.is_running && !app.is_stopped,
                "⏸ Pause",
                MUSTARD_YELLOW,
            );
            if pause_btn.clicked() {
                app.cmd_tx.send(DebuggerCommand::Pause).unwrap();
            }

            // Terminate
            let stop_btn = dbg_button(
                ui,
                app.is_running,
                "⏹ Stop",
                BRIGHT_RED,
            );
            if stop_btn.clicked() {
                app.cmd_tx.send(DebuggerCommand::Terminate).unwrap();
            }
        }
    });

    ui.separator();
    ui.horizontal(|ui| {
        ui.heading("Shellcode Disassembly");
        if !app.compiled_bytes.is_empty() {
            ui.add_space(8.0);
            let size_text = format!("({} bytes)", app.compiled_bytes.len());
            ui.label(
                egui::RichText::new(size_text)
                    .monospace()
                    .strong()
                    .color(CYBER_CYAN),
            );
        }
    });
    ui.separator();

    // Disassembly Instruction View
    let disasm_height = ui.available_height() - 250.0;
    egui::ScrollArea::vertical()
        .id_salt("disasm_scroll")
        .max_height(disasm_height)
        .show(ui, |ui| {
            if app.disassembly.is_empty() {
                ui.colored_label(Color32::GRAY, "(Assemble shellcode to view disassembly)");
            } else {
                let bad_chars = crate::assembler::parse_bad_characters(&app.bad_chars_input);
                let is_native =
                    !cfg!(target_arch = "wasm32") && app.target_arch == TargetArch::X86_64;
                let mut clicked_bp = None;
                let num_columns = if is_native { 6 } else { 4 };
                egui::Grid::new("disasm_grid")
                    .num_columns(num_columns)
                    .spacing([8.0, 4.0])
                    .show(ui, |ui| {
                        for inst in &app.disassembly {
                            let addr = inst.address as usize;
                            let is_current = app.is_running && app.regs.rip == inst.address;
                            let has_bp = app.breakpoints.contains(&addr);

                            if is_native {
                                // 1. Breakpoint margin toggle button
                                let bp_text = if has_bp { "🔴" } else { "  " };
                                let bp_rich = highlight_if(
                                    egui::RichText::new(bp_text).monospace(),
                                    is_current,
                               );
                                let bp_btn = ui.add(egui::Button::new(bp_rich).frame(false));
                                if bp_btn.clicked() {
                                    clicked_bp = Some((addr, has_bp));
                                }

                                // 2. Active RIP indicator arrow
                                let rip_indicator = if is_current { "👉" } else { "  " };
                                let rip_rich = highlight_if(
                                    egui::RichText::new(rip_indicator)
                                        .monospace()
                                        .strong()
                                        .color(CYBER_CYAN),
                                    is_current,
                                );
                                ui.label(rip_rich);
                            }

                            // 3. Instruction address
                            let addr_color = if is_current {
                                CYBER_CYAN
                            } else {
                                Color32::GRAY
                            };
                            let addr_rich = highlight_if(
                                egui::RichText::new(format!("0x{:08X}:", addr))
                                    .monospace()
                                    .color(addr_color),
                                is_current,
                            );
                            ui.label(addr_rich);

                            // 4. Hex machine bytes
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 4.0;
                                for &b in &inst.bytes {
                                    let is_bad = bad_chars.contains(&b);
                                    let b_color = if is_bad {
                                        BRIGHT_RED
                                    } else if is_current {
                                        ICE_BLUE
                                    } else {
                                        SLATE_GRAY
                                    };
                                    let mut text = egui::RichText::new(format!("{:02X}", b))
                                        .monospace()
                                        .color(b_color);
                                    if is_bad {
                                        text = text.strong();
                                    }
                                    let text = highlight_if(text, is_current);
                                    ui.label(text);
                                }
                            });

                            // 5. Mnemonic & Opcode operands
                            let text_color = if is_current {
                                CYBER_CYAN
                            } else {
                                Color32::WHITE
                            };
                            let mnem_rich = highlight_if(
                                egui::RichText::new(&inst.mnemonic)
                                    .monospace()
                                    .strong()
                                    .color(text_color),
                                is_current,
                            );
                            ui.label(mnem_rich);

                            let op_rich = highlight_if(
                                egui::RichText::new(&inst.op_str)
                                    .monospace()
                                    .color(text_color),
                                is_current,
                            );
                            ui.label(op_rich);
                            ui.end_row();
                        }
                    });

                if let Some((addr, has_bp)) = clicked_bp {
                    if has_bp {
                        app.breakpoints.remove(&addr);
                        app.cmd_tx
                            .send(DebuggerCommand::ToggleBreakpoint(addr, false))
                            .unwrap();
                        app.log(&format!("[Breakpoint] Removed at 0x{:08X}", addr));
                    } else {
                        app.breakpoints.insert(addr);
                        app.cmd_tx
                            .send(DebuggerCommand::ToggleBreakpoint(addr, true))
                            .unwrap();
                        app.log(&format!("[Breakpoint] Added at 0x{:08X}", addr));
                    }
                    app.sync_code_from_breakpoints();
                }
            }
        });

    ui.separator();

    // Logs Console with Tab layout
    ui.horizontal(|ui| {
        if ui
            .selectable_label(app.active_tab == ConsoleTab::Console, "Status Console")
            .clicked()
        {
            app.active_tab = ConsoleTab::Console;
        }
        if ui
            .selectable_label(app.active_tab == ConsoleTab::Stdout, "Stdout")
            .clicked()
        {
            app.active_tab = ConsoleTab::Stdout;
        }
        if ui
            .selectable_label(app.active_tab == ConsoleTab::Stderr, "Stderr")
            .clicked()
        {
            app.active_tab = ConsoleTab::Stderr;
        }
        if ui
            .selectable_label(app.active_tab == ConsoleTab::Shellcode, "Shellcode Output")
            .clicked()
        {
            app.active_tab = ConsoleTab::Shellcode;
        }
    });

    ui.separator();

    // Log Console text display
    if app.active_tab == ConsoleTab::Shellcode {
        egui::ScrollArea::vertical()
            .id_salt("shellcode_scroll")
            .max_height(200.0)
            .show(ui, |ui| {
                if app.compiled_bytes.is_empty() {
                    ui.colored_label(
                        Color32::GRAY,
                        "(Assemble shellcode to view formatted outputs)",
                    );
                } else {
                    let formats = [
                        ("Raw Hex String", app.format_raw_hex()),
                        ("Python Variable", app.format_python()),
                        ("C Variable", app.format_c()),
                        ("Rust Variable", app.format_rust()),
                    ];

                    for (label, val) in formats {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(label)
                                    .strong()
                                    .color(CYBER_CYAN),
                            );
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
                                .interactive(true),
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

        egui::ScrollArea::vertical()
            .id_salt("console_scroll")
            .max_height(200.0)
            .show(ui, |ui| {
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
                        .interactive(false),
                );
            });
    }
}
