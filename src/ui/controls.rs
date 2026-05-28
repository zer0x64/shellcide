use crate::app::{ConsoleTab, ShellcideApp};
use crate::debugger::DebuggerCommand;
use crate::ui::theme::{
    get_animation_factor, lerp_color, BRIGHT_RED, CYBER_CYAN, GLOOM_GRAY, ICE_BLUE, MUSTARD_YELLOW,
    NEON_PINK, SLATE_GRAY, TOXIC_GREEN,
};
use eframe::egui::{self, Color32};

fn dbg_button(ui: &mut egui::Ui, enabled: bool, text: &str, fill: Color32) -> egui::Response {
    let (fill_color, text_color) = if enabled {
        (fill, Color32::BLACK)
    } else {
        (GLOOM_GRAY, SLATE_GRAY)
    };
    ui.add_enabled(
        enabled,
        egui::Button::new(egui::RichText::new(text).strong().color(text_color))
            .min_size(egui::vec2(105.0, 24.0))
            .fill(fill_color),
    )
}

pub fn render_controls_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    let heading_text = if app.is_native_debug() {
        "Debugger Controls"
    } else {
        "Assembler Controls"
    };
    ui.heading(heading_text);
    ui.separator();

    // Command buttons
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        if app.is_native_debug() {
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
            let stop_btn = dbg_button(ui, app.is_running, "⏹ Stop", BRIGHT_RED);
            if stop_btn.clicked() {
                app.cmd_tx.send(DebuggerCommand::Terminate).unwrap();
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Assemble Code
            let compile_btn = dbg_button(ui, true, "⚙ Assemble", ICE_BLUE);
            if compile_btn.clicked() {
                app.do_assemble();
            }

            if let Some(status) = app.last_assemble_status {
                let alpha = get_animation_factor(app.last_assemble_time, 2.0, ui.ctx());
                if alpha > 0.0 {
                    let (text, color) = if status {
                        ("✓ SUCCESS", TOXIC_GREEN)
                    } else {
                        ("✗ FAILED", BRIGHT_RED)
                    };
                    let faded_color = Color32::from_rgba_unmultiplied(
                        color.r(),
                        color.g(),
                        color.b(),
                        (alpha * 255.0) as u8,
                    );
                    ui.colored_label(faded_color, text);
                }
            }
        });
    });

    ui.separator();

    ui.collapsing("🔒 Compression, Encryption & Encoding Pipeline", |ui| {
        egui::Grid::new("pipeline_grid")
            .num_columns(3)
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                // Compression
                ui.label("Compression:");
                egui::ComboBox::from_id_salt("compression_combo")
                    .selected_text(app.compression_type.display_name())
                    .show_ui(ui, |ui| {
                        for ct in &crate::encoder::CompressionType::ALL {
                            ui.selectable_value(&mut app.compression_type, *ct, ct.display_name());
                        }
                    });
                if app.compression_type != crate::encoder::CompressionType::None {
                    if let Some(marker) = app.resolved_compression_marker {
                        ui.colored_label(TOXIC_GREEN, format!("Resolved Marker: 0x{:02X}", marker));
                    } else {
                        ui.colored_label(MUSTARD_YELLOW, "Resolved Marker: None");
                    }
                } else {
                    ui.weak("No compression selected");
                }
                ui.end_row();

                // Encryption
                ui.label("Encryption:");
                egui::ComboBox::from_id_salt("encryption_combo")
                    .selected_text(app.encryption_type.display_name())
                    .show_ui(ui, |ui| {
                        for et in &crate::encoder::EncryptionType::ALL {
                            ui.selectable_value(&mut app.encryption_type, *et, et.display_name());
                        }
                    });
                ui.horizontal(|ui| {
                    if app.encryption_type != crate::encoder::EncryptionType::None {
                        ui.label("Key:");
                        ui.add(
                            egui::TextEdit::singleline(&mut app.encryption_key)
                                .hint_text("leave empty for random"),
                        )
                        .on_hover_text("Leave key empty to automatically generate a random one");
                    } else {
                        ui.weak("No encryption selected");
                    }
                });
                ui.end_row();

                // Encoding
                ui.label("Encoding:");
                egui::ComboBox::from_id_salt("encoding_combo")
                    .selected_text(app.encoding_type.display_name())
                    .show_ui(ui, |ui| {
                        for et in &crate::encoder::EncodingType::ALL {
                            ui.selectable_value(&mut app.encoding_type, *et, et.display_name());
                        }
                    });
                if app.encoding_type != crate::encoder::EncodingType::None {
                    if let Some(key) = app.resolved_encoding_key {
                        ui.colored_label(TOXIC_GREEN, format!("Resolved Key: 0x{:02X}", key));
                    } else {
                        ui.colored_label(MUSTARD_YELLOW, "Resolved Key: None");
                    }
                } else {
                    ui.weak("No encoding selected");
                }
                ui.end_row();
            });
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
    let flash_factor = get_animation_factor(app.last_disasm_change_time, 0.5, ui.ctx());
    let border_color = lerp_color(GLOOM_GRAY, CYBER_CYAN, flash_factor);

    egui::Frame::none()
        .stroke(egui::Stroke::new(1.0, border_color))
        .inner_margin(4.0)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .id_salt("disasm_scroll")
                .show(ui, |ui| {
                    if app.disassembly.is_empty() {
                        ui.colored_label(Color32::GRAY, "(Assemble shellcode to view disassembly)");
                    } else {
                        let bad_chars =
                            crate::assembler::parse_bad_characters(&app.bad_chars_input);
                        let is_native = app.is_native_debug();
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

                                    // Closure for formatting rich text in the current row context
                                    let fmt_text = |val: &str, color: Color32, strong: bool| {
                                        let mut r =
                                            egui::RichText::new(val).monospace().color(color);
                                        if strong {
                                            r = r.strong();
                                        }
                                        if is_current {
                                            r = r.background_color(
                                                Color32::from_rgba_unmultiplied(
                                                    CYBER_CYAN.r(),
                                                    CYBER_CYAN.g(),
                                                    CYBER_CYAN.b(),
                                                    30,
                                                ),
                                            );
                                        }
                                        r
                                    };

                                    if is_native {
                                        // 1. Breakpoint margin toggle button
                                        let bp_text = if has_bp { "🔴" } else { "  " };
                                        let bp_btn = ui.add(
                                            egui::Button::new(fmt_text(
                                                bp_text,
                                                Color32::WHITE,
                                                false,
                                            ))
                                            .frame(false),
                                        );
                                        if bp_btn.clicked() {
                                            clicked_bp = Some((addr, has_bp));
                                        }

                                        // 2. Active RIP indicator arrow
                                        let rip_indicator = if is_current { "👉" } else { "  " };
                                        ui.label(fmt_text(rip_indicator, CYBER_CYAN, true));
                                    }

                                    // 3. Instruction address
                                    let addr_color = if is_current {
                                        CYBER_CYAN
                                    } else {
                                        Color32::GRAY
                                    };
                                    ui.label(fmt_text(
                                        &format!("0x{:08X}:", addr),
                                        addr_color,
                                        false,
                                    ));

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
                                            ui.label(fmt_text(
                                                &format!("{:02X}", b),
                                                b_color,
                                                is_bad,
                                            ));
                                        }
                                    });

                                    // 5. Mnemonic & Opcode operands
                                    let text_color = if is_current {
                                        CYBER_CYAN
                                    } else {
                                        Color32::WHITE
                                    };
                                    ui.label(fmt_text(&inst.mnemonic, text_color, true));
                                    ui.label(fmt_text(&inst.op_str, text_color, false));
                                    ui.end_row();
                                }
                            });

                        if let Some((addr, _has_bp)) = clicked_bp {
                            app.toggle_breakpoint(addr);
                            app.sync_code_from_breakpoints();
                        }
                    }
                });
        });
}

pub fn render_console_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    // Logs Console with Tab layout
    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.active_tab, ConsoleTab::Console, "Status Console");
        ui.selectable_value(&mut app.active_tab, ConsoleTab::Stdout, "Stdout");
        ui.selectable_value(&mut app.active_tab, ConsoleTab::Stderr, "Stderr");
        ui.selectable_value(
            &mut app.active_tab,
            ConsoleTab::Shellcode,
            "Shellcode Output",
        );
    });

    ui.separator();

    // Log Console text display
    if app.active_tab == ConsoleTab::Shellcode {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .id_salt("shellcode_scroll")
            .show(ui, |ui| {
                if app.compiled_bytes.is_empty() {
                    ui.colored_label(
                        Color32::GRAY,
                        "(Assemble shellcode to view formatted outputs)",
                    );
                } else {
                    let formats = [
                        ("Raw Hex String", app.format_raw_hex(&app.compiled_bytes)),
                        ("Python Variable", app.format_python(&app.compiled_bytes)),
                        ("C Variable", app.format_c(&app.compiled_bytes)),
                        ("Rust Variable", app.format_rust(&app.compiled_bytes)),
                    ];

                    for (label, val) in formats {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(label).strong().color(CYBER_CYAN));
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
        let mut dummy;
        let active_log = match app.active_tab {
            ConsoleTab::Console => {
                if app.console_log.is_empty() {
                    dummy = "(empty)".to_string();
                    &mut dummy
                } else {
                    &mut app.console_log
                }
            }
            ConsoleTab::Stdout => {
                if app.stdout_log.is_empty() {
                    dummy = "(empty)".to_string();
                    &mut dummy
                } else {
                    &mut app.stdout_log
                }
            }
            ConsoleTab::Stderr => {
                if app.stderr_log.is_empty() {
                    dummy = "(empty)".to_string();
                    &mut dummy
                } else {
                    &mut app.stderr_log
                }
            }
            ConsoleTab::Shellcode => unreachable!(),
        };

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .id_salt("console_scroll")
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(active_log)
                        .font(egui::FontId::monospace(12.0))
                        .desired_width(f32::INFINITY)
                        .desired_rows(8)
                        .interactive(false),
                );
            });
    }
}
