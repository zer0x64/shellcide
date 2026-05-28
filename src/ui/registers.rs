use crate::app::ShellcideApp;
use crate::debugger::DebuggerCommand;
use crate::ui::theme::{get_animation_factor, lerp_color, CYBER_CYAN, SOFT_ORANGE};
use chrono::Utc;
use eframe::egui::{self, Color32};

pub fn render_registers_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("Registers");
    ui.separator();

    // RFlags flags breakdown
    ui.horizontal(|ui| {
        ui.label("RFLAGS Bits:");
        let rflags = app.regs.rflags;
        let flags = [
            ("ZF", (rflags & 0x0040) != 0),
            ("CF", (rflags & 0x0001) != 0),
            ("SF", (rflags & 0x0080) != 0),
            ("OF", (rflags & 0x0800) != 0),
            ("PF", (rflags & 0x0004) != 0),
            ("AF", (rflags & 0x0010) != 0),
            ("TF", (rflags & 0x0100) != 0),
            ("IF", (rflags & 0x0200) != 0),
            ("DF", (rflags & 0x0400) != 0),
        ];
        for (name, val) in flags {
            let color = if val { CYBER_CYAN } else { Color32::GRAY };
            ui.colored_label(color, name);
        }
    });

    ui.separator();

    // Register Grid
    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .id_salt("regs_scroll")
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
                    let ctx = ui.ctx().clone();
                    for (idx, &(name, val)) in reg_list.iter().enumerate() {
                        let name_str = name.to_string();

                        let animation_factor = get_animation_factor(
                            app.reg_change_times.get(name).copied(),
                            0.8,
                            ui.ctx(),
                        );

                        let name_color = lerp_color(CYBER_CYAN, SOFT_ORANGE, animation_factor);

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
                                    app.reg_change_times.insert(name.to_string(), Utc::now());
                                } else {
                                    app.log("[Input Error] Invalid register integer format.");
                                }
                                app.editing_register = None;
                            }
                        } else {
                            let label_text = format!("0x{:016X}", val);
                            let val_color =
                                lerp_color(Color32::WHITE, SOFT_ORANGE, animation_factor);
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

                        if idx % 2 == 1 {
                            ui.end_row();
                        } else {
                            ui.separator();
                        }
                    }
                });
        });
}
