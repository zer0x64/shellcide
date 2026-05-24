use eframe::egui::{self, Color32};
use crate::app::{ShellcideApp, TargetArch};

pub fn render_header_panel(app: &mut ShellcideApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let title = match app.target_arch {
                TargetArch::X86_64 => "SHELLCIDE // X86_64 IDE & DEBUGGER".to_string(),
                other => format!("SHELLCIDE // {:?} IDE", other).to_uppercase(),
            };
            ui.heading(
                egui::RichText::new(title)
                    .color(Color32::from_rgb(0, 206, 201))
                    .strong()
            );
            
            ui.separator();
            
            // File Operations
            ui.label("File:");
            ui.add(egui::TextEdit::singleline(&mut app.active_path).desired_width(120.0));
            
            if ui.button("📁 Load").clicked() {
                match std::fs::read_to_string(&app.active_path) {
                    Ok(content) => {
                        app.code_input = content;
                        app.log(&format!("[File] Loaded file: {}", app.active_path));
                    }
                    Err(e) => app.log(&format!("[File System Error] Failed to load: {}", e)),
                }
            }
            
            if ui.button("💾 Save").clicked() {
                match std::fs::write(&app.active_path, &app.code_input) {
                    Ok(_) => app.log(&format!("[File] Saved to: {}", app.active_path)),
                    Err(e) => app.log(&format!("[File System Error] Failed to save: {}", e)),
                }
            }

            ui.separator();

            // Syntax toggle
            ui.label("Syntax style:");
            let old_att = app.att_syntax;
            ui.checkbox(&mut app.att_syntax, "AT&T (GNU syntax)");
            if old_att != app.att_syntax {
                app.log(&format!("[Syntax] Switched to {}", if app.att_syntax { "AT&T" } else { "Intel" }));
            }

            ui.separator();

            // Architecture selector
            ui.label("Arch:");
            let old_arch = app.target_arch;
            egui::ComboBox::from_id_salt("arch_combo")
                .selected_text(format!("{:?}", app.target_arch))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.target_arch, TargetArch::X86_64, "X86_64");
                    ui.selectable_value(&mut app.target_arch, TargetArch::X86, "X86");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Arm, "Arm");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Thumb, "Thumb");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Aarch64, "Aarch64");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Riscv, "Riscv");
                });
            if old_arch != app.target_arch {
                app.log(&format!("[Arch] Switched architecture to {:?}", app.target_arch));
                app.do_assemble();
            }
        });
    });
}
