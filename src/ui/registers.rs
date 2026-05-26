use crate::app::ShellcideApp;
use crate::debugger::DebuggerCommand;
use eframe::egui::{self, Color32};

fn lerp_color(from: Color32, to: Color32, t: f32) -> Color32 {
    let r = (from.r() as f32 + (to.r() as f32 - from.r() as f32) * t).round() as u8;
    let g = (from.g() as f32 + (to.g() as f32 - from.g() as f32) * t).round() as u8;
    let b = (from.b() as f32 + (to.b() as f32 - from.b() as f32) * t).round() as u8;
    Color32::from_rgb(r, g, b)
}

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
        ui.colored_label(
            if zf {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "ZF",
        );
        ui.colored_label(
            if cf {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "CF",
        );
        ui.colored_label(
            if sf {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "SF",
        );
        ui.colored_label(
            if of {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "OF",
        );
        ui.colored_label(
            if pf {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "PF",
        );
        ui.colored_label(
            if af {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "AF",
        );
        ui.colored_label(
            if tf {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "TF",
        );
        ui.colored_label(
            if if_ {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "IF",
        );
        ui.colored_label(
            if df {
                Color32::from_rgb(0, 206, 201)
            } else {
                Color32::GRAY
            },
            "DF",
        );
    });

    ui.separator();

    // Register Grid
    egui::ScrollArea::both()
        .id_salt("regs_scroll")
        .max_height(250.0)
        .show(ui, |ui| {
            let reg_list = [
                ("rax", app.regs.rax),
                ("rbx", app.regs.rbx),
                ("rcx", app.regs.rcx),
                ("rdx", app.regs.rdx),
                ("rsi", app.regs.rsi),
                ("rdi", app.regs.rdi),
                ("rbp", app.regs.rbp),
                ("rsp", app.regs.rsp),
                ("rip", app.regs.rip),
                ("rflags", app.regs.rflags),
                ("r8", app.regs.r8),
                ("r9", app.regs.r9),
                ("r10", app.regs.r10),
                ("r11", app.regs.r11),
                ("r12", app.regs.r12),
                ("r13", app.regs.r13),
                ("r14", app.regs.r14),
                ("r15", app.regs.r15),
            ];

            egui::Grid::new("registers_grid")
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    let mut idx = 0;
                    let ctx = ui.ctx().clone();
                    for &(name, val) in &reg_list {
                        let name_str = name.to_string();

                        let animation_factor =
                            if let Some(last_changed) = app.reg_change_times.get(name) {
                                let elapsed = last_changed.elapsed().as_secs_f32();
                                let duration = 0.8; // 800ms fade duration
                                if elapsed < duration {
                                    ui.ctx().request_repaint();
                                    1.0 - (elapsed / duration)
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            };

                        let name_color = lerp_color(
                            Color32::from_rgb(0, 206, 201),
                            Color32::from_rgb(250, 177, 160),
                            animation_factor,
                        );

                        // Draw register name
                        ui.label(egui::RichText::new(name).color(name_color).strong());

                        // Draw editable value (double-click or click to edit)
                        if app.editing_register.as_ref() == Some(&name_str) {
                            let text_edit = ui.add(
                                egui::TextEdit::singleline(&mut app.register_input)
                                    .font(egui::FontId::monospace(14.0))
                                    .desired_width(120.0),
                            );
                            if text_edit.lost_focus()
                                || ctx.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                if let Some(v) = crate::ui::parse_u64_input(&app.register_input) {
                                    app.regs.set_by_name(name, v);
                                    if app.is_running && app.is_stopped {
                                        app.cmd_tx
                                            .send(DebuggerCommand::WriteRegs(app.regs))
                                            .unwrap();
                                        app.log(&format!(
                                            "[Debugger] Updated register {} to 0x{:X}",
                                            name, v
                                        ));
                                    }
                                    app.reg_change_times
                                        .insert(name.to_string(), std::time::Instant::now());
                                } else {
                                    app.log("[Input Error] Invalid register integer format.");
                                }
                                app.editing_register = None;
                            }
                        } else {
                            let label_text = format!("0x{:016X}", val);
                            let val_color = lerp_color(
                                Color32::WHITE,
                                Color32::from_rgb(250, 177, 160),
                                animation_factor,
                            );
                            let val_label = ui.add(
                                egui::Label::new(
                                    egui::RichText::new(label_text).color(val_color).monospace(),
                                )
                                .sense(egui::Sense::click()),
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
