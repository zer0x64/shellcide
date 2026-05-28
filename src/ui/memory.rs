use crate::app::ShellcideApp;
use crate::debugger::DebuggerCommand;
use crate::ui::theme::{
    get_animation_factor, lerp_color, CYBER_CYAN, ICE_BLUE, LAUGHTER_PURPLE, NEON_PINK, SLATE_GRAY,
    SOFT_ORANGE,
};
use eframe::egui::{self, Color32};

fn with_alpha(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

fn highlight_stack_cell(
    mut label: egui::RichText,
    byte_addr: usize,
    app: &ShellcideApp,
) -> egui::RichText {
    if app.is_running && app.is_stopped {
        let rsp = app.regs.rsp as usize;
        let rbp = app.regs.rbp as usize;
        if byte_addr == rsp {
            label = label.background_color(with_alpha(NEON_PINK, 120));
        } else if byte_addr == rbp {
            label = label.background_color(with_alpha(LAUGHTER_PURPLE, 120));
        } else if (rsp + 1..rbp).contains(&byte_addr) {
            label = label.background_color(with_alpha(NEON_PINK, 30));
        }
    }
    label
}

pub fn render_memory_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("Memory Editor");
    ui.separator();

    // Base address configuration
    ui.horizontal(|ui| {
        ui.label("Segment Address:");
        let text_edit =
            ui.add(egui::TextEdit::singleline(&mut app.mem_base_input).desired_width(120.0));

        if text_edit.lost_focus() || ui.button("Go").clicked() {
            if let Some(addr) = crate::ui::parse_u64_input(&app.mem_base_input).map(|v| v as usize)
            {
                app.memory_base_address = addr;
                app.refresh_memory();
            } else {
                app.log("[Input Error] Invalid address format.");
            }
        }

        if app.is_running && app.is_stopped {
            if ui.button("Jump to RSP").clicked() {
                app.jump_to_memory_address(app.get_centered_rsp());
            }
            if ui.button("Jump to RBP").clicked() {
                app.jump_to_memory_address(app.regs.rbp as usize);
            }
            ui.checkbox(&mut app.auto_follow_rsp, "Auto-Follow RSP");
        }
    });

    ui.separator();

    // Hex Memory Grid
    egui::ScrollArea::both().auto_shrink([false, false]).id_salt("mem_scroll").show(ui, |ui| {
        egui::Grid::new("mem_grid").spacing([4.0, 4.0]).show(ui, |ui| {
            // Header row
            ui.label(egui::RichText::new("Offset").strong().monospace());
            for i in 0..16 {
                ui.label(egui::RichText::new(format!("{:02X}", i)).strong().monospace());
            }
            ui.label(egui::RichText::new("ASCII").strong().monospace());
            ui.end_row();

            // Render rows (16 rows of 16 bytes = 256 bytes)
            let ctx = ui.ctx().clone();
            for r in 0..16 {
                let row_offset = r * 16;
                let row_addr = app.memory_base_address + row_offset;
                let mut suffix = String::new();
                if app.is_running && app.is_stopped {
                    let rsp = app.regs.rsp as usize;
                    let rbp = app.regs.rbp as usize;
                    let has_rsp = (row_addr..row_addr + 16).contains(&rsp);
                    let has_rbp = (row_addr..row_addr + 16).contains(&rbp);
                    match (has_rsp, has_rbp) {
                        (true, true) => suffix.push_str(" [RSP,RBP]"),
                        (true, false) => suffix.push_str(" [RSP]"),
                        (false, true) => suffix.push_str(" [RBP]"),
                        _ => {}
                    }
                }

                // Precompute flash factors for each byte in the row to avoid redundant lookups/calculations
                let mut flash_factors = [0.0; 16];
                for (c, factor) in flash_factors.iter_mut().enumerate() {
                    let byte_addr = row_addr + c;
                    *factor = get_animation_factor(
                        app.mem_change_times.get(&byte_addr).copied(),
                        0.8,
                        ui.ctx(),
                    );
                }

                // Address offset label
                ui.label(egui::RichText::new(format!("0x{:08X}{}", row_addr, suffix)).monospace().color(CYBER_CYAN));

                // Hex bytes
                for (c, &flash_factor) in flash_factors.iter().enumerate() {
                    let idx = row_offset + c;
                    let byte_val = app.memory_data.get(idx).copied().unwrap_or(0);
                    let byte_addr = row_addr + c;

                    if app.editing_memory_byte == Some(idx) {
                        // Double click edit view
                        let text_edit = ui.add(
                            egui::TextEdit::singleline(&mut app.memory_byte_input)
                                .font(egui::FontId::monospace(14.0))
                                .desired_width(20.0)
                        );
                        if text_edit.lost_focus() || ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                            if let Ok(v) = u8::from_str_radix(app.memory_byte_input.trim(), 16) {
                                if let Some(b) = app.memory_data.get_mut(idx) {
                                    *b = v;
                                }
                                if app.is_running && app.is_stopped {
                                    app.cmd_tx.send(DebuggerCommand::WriteMemory(byte_addr, vec![v])).unwrap();
                                    app.log(&format!("[Debugger] Wrote byte 0x{:02X} to memory address 0x{:X}", v, byte_addr));
                                    if (crate::debugger::CODE_BASE..crate::debugger::CODE_BASE + app.compiled_bytes.len()).contains(&byte_addr) {
                                        app.refresh_disassembly_from_running_process();
                                    }
                                }
                            }
                            app.editing_memory_byte = None;
                        }
                    } else {
                        let label_text = format!("{:02X}", byte_val);
                        let default_color = if byte_val == 0 { SLATE_GRAY } else { Color32::WHITE };
                        let label_color = lerp_color(default_color, SOFT_ORANGE, flash_factor);
                        let label = egui::RichText::new(label_text).monospace().color(label_color);
                        let label = highlight_stack_cell(label, byte_addr, app);
                        let byte_label = ui.add(
                            egui::Label::new(label)
                                .sense(egui::Sense::click())
                        );
                        if byte_label.clicked() {
                            app.editing_memory_byte = Some(idx);
                            app.memory_byte_input = format!("{:02X}", byte_val);
                        }
                    }
                }

                // ASCII representation
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    for (c, &flash_factor) in flash_factors.iter().enumerate() {
                        let idx = row_offset + c;
                        let b = app.memory_data.get(idx).copied().unwrap_or(0);
                        let byte_addr = row_addr + c;
                        let char_val = if (32..=126).contains(&b) { b as char } else { '.' };
                        let label_color = lerp_color(ICE_BLUE, SOFT_ORANGE, flash_factor);
                        let label = egui::RichText::new(char_val.to_string()).monospace().color(label_color);
                        let label = highlight_stack_cell(label, byte_addr, app);
                        ui.label(label);
                    }
                });
                ui.end_row();
            }
        });
    });
}
