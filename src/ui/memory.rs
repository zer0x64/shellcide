use eframe::egui::{self, Color32};
use crate::app::ShellcideApp;
use crate::debugger::DebuggerCommand;

fn highlight_stack_cell(mut label: egui::RichText, byte_addr: usize, app: &ShellcideApp) -> egui::RichText {
    if app.is_running && app.is_stopped {
        let rsp = app.regs.rsp as usize;
        let rbp = app.regs.rbp as usize;
        if byte_addr == rsp {
            label = label.background_color(Color32::from_rgba_unmultiplied(253, 121, 168, 120));
        } else if byte_addr == rbp {
            label = label.background_color(Color32::from_rgba_unmultiplied(162, 155, 254, 120));
        } else if byte_addr > rsp && byte_addr < rbp {
            label = label.background_color(Color32::from_rgba_unmultiplied(253, 121, 168, 30));
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
        let text_edit = ui.add(egui::TextEdit::singleline(&mut app.mem_base_input).desired_width(120.0));
        
        if text_edit.lost_focus() || ui.button("Go").clicked() {
            let clean = app.mem_base_input.trim();
            let parsed = if clean.starts_with("0x") || clean.starts_with("0X") {
                usize::from_str_radix(&clean[2..], 16).ok()
            } else {
                clean.parse::<usize>().ok().or_else(|| usize::from_str_radix(clean, 16).ok())
            };

            if let Some(addr) = parsed {
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
    egui::ScrollArea::both().id_salt("mem_scroll").show(ui, |ui| {
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
                    let has_rsp = rsp >= row_addr && rsp < row_addr + 16;
                    let has_rbp = rbp >= row_addr && rbp < row_addr + 16;
                    if has_rsp && has_rbp {
                        suffix.push_str(" [RSP,RBP]");
                    } else if has_rsp {
                        suffix.push_str(" [RSP]");
                    } else if has_rbp {
                        suffix.push_str(" [RBP]");
                    }
                }

                // Address offset label
                ui.label(egui::RichText::new(format!("0x{:08X}{}", row_addr, suffix)).monospace().color(Color32::from_rgb(0, 206, 201)));

                // Hex bytes
                for c in 0..16 {
                    let idx = row_offset + c;
                    let byte_val = app.memory_data[idx];
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
                                app.memory_data[idx] = v;
                                if app.is_running && app.is_stopped {
                                    app.cmd_tx.send(DebuggerCommand::WriteMemory(byte_addr, vec![v])).unwrap();
                                    app.log(&format!("[Debugger] Wrote byte 0x{:02X} to memory address 0x{:X}", v, byte_addr));
                                }
                            }
                            app.editing_memory_byte = None;
                        }
                    } else {
                        let label_text = format!("{:02X}", byte_val);
                        let label_color = if byte_val == 0 { Color32::from_rgb(99, 110, 114) } else { Color32::WHITE };
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
                    for c in 0..16 {
                        let idx = row_offset + c;
                        let b = app.memory_data[idx];
                        let byte_addr = row_addr + c;
                        let char_str = if (32..=126).contains(&b) {
                            (b as char).to_string()
                        } else {
                            ".".to_string()
                        };
                        let label = egui::RichText::new(char_str).monospace().color(Color32::from_rgb(116, 185, 255));
                        let label = highlight_stack_cell(label, byte_addr, app);
                        ui.label(label);
                    }
                });
                ui.end_row();
            }
        });
    });
}
