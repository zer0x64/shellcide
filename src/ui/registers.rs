use eframe::egui::{self, Color32};
use crate::app::ShellcideApp;
use crate::debugger::DebuggerCommand;

pub fn render_registers_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("Registers");
    ui.separator();

    // RFlags flags breakdown
    ui.horizontal(|ui| {
        let rflags = app.regs.rflags;
        let cf = (rflags & 0x0001) != 0;
        let pf = (rflags & 0x0004) != 0;
        let af = (rflags & 0x0010) != 0;
        let zf = (rflags & 0x0040) != 0;
        let sf = (rflags & 0x0080) != 0;
        let tf = (rflags & 0x0100) != 0;
        let if_ = (rflags & 0x0200) != 0;
        let df = (rflags & 0x0400) != 0;
        let of = (rflags & 0x0800) != 0;

        ui.label("RFLAGS Bits:");
        ui.colored_label(if zf { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "ZF");
        ui.colored_label(if cf { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "CF");
        ui.colored_label(if sf { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "SF");
        ui.colored_label(if of { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "OF");
        ui.colored_label(if pf { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "PF");
        ui.colored_label(if af { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "AF");
        ui.colored_label(if tf { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "TF");
        ui.colored_label(if if_ { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "IF");
        ui.colored_label(if df { Color32::from_rgb(0, 206, 201) } else { Color32::GRAY }, "DF");
    });

    ui.separator();

    // Register Grid
    egui::ScrollArea::both().id_salt("regs_scroll").max_height(250.0).show(ui, |ui| {
        let reg_list = vec![
            ("rax", app.regs.rax), ("rbx", app.regs.rbx),
            ("rcx", app.regs.rcx), ("rdx", app.regs.rdx),
            ("rsi", app.regs.rsi), ("rdi", app.regs.rdi),
            ("rbp", app.regs.rbp), ("rsp", app.regs.rsp),
            ("rip", app.regs.rip), ("rflags", app.regs.rflags),
            ("r8", app.regs.r8), ("r9", app.regs.r9),
            ("r10", app.regs.r10), ("r11", app.regs.r11),
            ("r12", app.regs.r12), ("r13", app.regs.r13),
            ("r14", app.regs.r14), ("r15", app.regs.r15),
        ];

        egui::Grid::new("registers_grid").striped(true).num_columns(4).show(ui, |ui| {
            let mut idx = 0;
            let ctx = ui.ctx().clone();
            for (name, val) in reg_list.clone() {
                let name_str = name.to_string();
                let changed = app.previous_regs.is_some_and(|p| {
                    match name {
                        "rax" => p.rax != val, "rbx" => p.rbx != val,
                        "rcx" => p.rcx != val, "rdx" => p.rdx != val,
                        "rsi" => p.rsi != val, "rdi" => p.rdi != val,
                        "rbp" => p.rbp != val, "rsp" => p.rsp != val,
                        "rip" => p.rip != val, "rflags" => p.rflags != val,
                        "r8" => p.r8 != val, "r9" => p.r9 != val,
                        "r10" => p.r10 != val, "r11" => p.r11 != val,
                        "r12" => p.r12 != val, "r13" => p.r13 != val,
                        "r14" => p.r14 != val, "r15" => p.r15 != val,
                        _ => false,
                    }
                });

                // Draw register name
                ui.label(
                    egui::RichText::new(name)
                        .color(if changed { Color32::from_rgb(250, 177, 160) } else { Color32::from_rgb(0, 206, 201) })
                        .strong()
                );

                // Draw editable value (double-click or click to edit)
                if app.editing_register.as_ref() == Some(&name_str) {
                    let text_edit = ui.add(
                        egui::TextEdit::singleline(&mut app.register_input)
                            .font(egui::FontId::monospace(14.0))
                            .desired_width(120.0)
                    );
                    if text_edit.lost_focus() || ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let mut parsed_val = None;
                        let clean_input = app.register_input.trim();
                        if clean_input.starts_with("0x") || clean_input.starts_with("0X") {
                            if let Ok(v) = u64::from_str_radix(&clean_input[2..], 16) {
                                parsed_val = Some(v);
                            }
                        } else if clean_input.ends_with('h') || clean_input.ends_with('H') {
                            if let Ok(v) = u64::from_str_radix(&clean_input[..clean_input.len()-1], 16) {
                                parsed_val = Some(v);
                            }
                        } else {
                            if let Ok(v) = clean_input.parse::<u64>() {
                                parsed_val = Some(v);
                            } else if let Ok(v) = u64::from_str_radix(clean_input, 16) {
                                parsed_val = Some(v);
                            }
                        }

                        if let Some(v) = parsed_val {
                            match name {
                                "rax" => app.regs.rax = v, "rbx" => app.regs.rbx = v,
                                "rcx" => app.regs.rcx = v, "rdx" => app.regs.rdx = v,
                                "rsi" => app.regs.rsi = v, "rdi" => app.regs.rdi = v,
                                "rbp" => app.regs.rbp = v, "rsp" => app.regs.rsp = v,
                                "rip" => app.regs.rip = v, "rflags" => app.regs.rflags = v,
                                "r8" => app.regs.r8 = v, "r9" => app.regs.r9 = v,
                                "r10" => app.regs.r10 = v, "r11" => app.regs.r11 = v,
                                "r12" => app.regs.r12 = v, "r13" => app.regs.r13 = v,
                                "r14" => app.regs.r14 = v, "r15" => app.regs.r15 = v,
                                _ => {}
                            }
                            if app.is_running && app.is_stopped {
                                app.cmd_tx.send(DebuggerCommand::WriteRegs(app.regs)).unwrap();
                                app.log(&format!("[Debugger] Updated register {} to 0x{:X}", name, v));
                            }
                        } else {
                            app.log("[Input Error] Invalid register integer format.");
                        }
                        app.editing_register = None;
                    }
                } else {
                    let label_text = format!("0x{:016X}", val);
                    let label_color = if changed { Color32::from_rgb(250, 177, 160) } else { Color32::WHITE };
                    let val_label = ui.add(
                        egui::Label::new(egui::RichText::new(label_text).color(label_color).monospace())
                            .sense(egui::Sense::click())
                    );
                    if val_label.clicked() {
                        app.editing_register = Some(name_str);
                        app.register_input = format!("0x{:X}", val);
                    }
                }

                idx += 1;
                if idx % 2 == 0 {
                    ui.end_row();
                } else {
                    ui.separator();
                }
            }
        });
    });
}
