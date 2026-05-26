use crate::app::ShellcideApp;
use eframe::egui::{self, Color32};

pub(crate) fn extract_arg_name(arg_type: &str) -> &str {
    let trimmed = arg_type.trim();
    if trimmed.is_empty() {
        return "";
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return "";
    }
    let last = parts[parts.len() - 1];
    let mut name = last;
    while name.starts_with('*') {
        name = &name[1..];
    }
    if name.is_empty() {
        for part in parts.iter().rev().skip(1) {
            let mut p = *part;
            while p.starts_with('*') {
                p = &p[1..];
            }
            if !p.is_empty() && p != "const" && p != "struct" {
                return p;
            }
        }
        return arg_type;
    }
    name
}

pub fn render_syscalls_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("Linux x86_64 Syscalls");
    ui.horizontal(|ui| {
        ui.label("🔍");
        ui.add(
            egui::TextEdit::singleline(&mut app.syscall_search)
                .hint_text("Search...")
                .desired_width(120.0),
        );
        if ui.button("Clear").clicked() {
            app.syscall_search.clear();
        }
    });
    ui.separator();

    let search = app.syscall_search.trim().to_lowercase();

    egui::ScrollArea::both()
        .id_salt("syscalls_scroll")
        .show(ui, |ui| {
            egui::Grid::new("syscall_grid")
                .striped(true)
                .num_columns(9)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Drag")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("RAX")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("Name")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("RDI (1st)")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("RSI (2nd)")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("RDX (3rd)")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("R10 (4th)")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("R8 (5th)")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.label(
                        egui::RichText::new("R9 (6th)")
                            .strong()
                            .color(Color32::from_rgb(0, 206, 201)),
                    );
                    ui.end_row();

                    let get_arg_by_reg =
                        |s: &crate::syscalls::SyscallInfo, reg_name: &str| -> &str {
                            s.args
                                .iter()
                                .find(|arg| arg.reg == reg_name)
                                .map(|arg| extract_arg_name(arg.arg_type))
                                .unwrap_or("")
                        };

                    for s in crate::syscalls::SYSCALLS.iter() {
                        if !search.is_empty() {
                            let matches_name = s.name.to_lowercase().contains(&search);
                            let matches_nr = s.nr.to_string().contains(&search);
                            let matches_entry = s.entry_point.to_lowercase().contains(&search);
                            let matches_args = s.args.iter().any(|arg| {
                                arg.reg.to_lowercase().contains(&search)
                                    || arg.arg_type.to_lowercase().contains(&search)
                            });
                            if !(matches_name || matches_nr || matches_entry || matches_args) {
                                continue;
                            }
                        }

                        // Drag handle
                        let payload = s.name.to_string();
                        let item_id = egui::Id::new(format!("dnd_syscall_{}", s.nr));
                        ui.dnd_drag_source(item_id, payload, |ui| {
                            ui.label(
                                egui::RichText::new("⠿")
                                    .monospace()
                                    .color(Color32::from_rgb(0, 206, 201)),
                            );
                        });

                        // RAX
                        ui.label(egui::RichText::new(s.nr.to_string()).monospace());

                        // Name / Man
                        if !s.man_url.is_empty() {
                            ui.hyperlink_to(egui::RichText::new(s.name).strong(), s.man_url);
                        } else {
                            ui.label(egui::RichText::new(s.name).strong());
                        }

                        // Arguments (RDI, RSI, RDX, R10, R8, R9)
                        let regs = ["%rdi", "%rsi", "%rdx", "%r10", "%r8", "%r9"];
                        for reg in regs {
                            let arg = get_arg_by_reg(s, reg);
                            if arg.is_empty() {
                                ui.label(egui::RichText::new("-").weak());
                            } else {
                                ui.label(egui::RichText::new(arg).monospace());
                            }
                        }

                        ui.end_row();
                    }
                });
        });
}
