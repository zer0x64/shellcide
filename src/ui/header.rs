use crate::app::{ShellcideApp, TargetArch};
use crate::ui::theme::CYBER_CYAN;
use eframe::egui;

pub fn render_header_panel(app: &mut ShellcideApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let suffix = if app.target_arch == TargetArch::X86_64 {
                " & DEBUGGER"
            } else {
                ""
            };
            let title = format!(
                "SHELLCIDE // {} IDE{}",
                app.target_arch.display_name(),
                suffix
            )
            .to_uppercase();
            ui.heading(egui::RichText::new(title).color(CYBER_CYAN).strong());

            ui.separator();

            // File Operations
            ui.label("File:");
            ui.add(egui::TextEdit::singleline(&mut app.active_path).desired_width(120.0));

            if ui.button("📁 Load").clicked() {
                app.load_file();
            }
            if ui.button("💾 Save").clicked() {
                app.save_file();
            }

            ui.separator();

            // Syntax toggle
            ui.label("Syntax style:");
            let old_att = app.att_syntax;
            ui.checkbox(&mut app.att_syntax, "AT&T (GNU syntax)");
            if old_att != app.att_syntax {
                app.log(&format!(
                    "[Syntax] Switched to {}",
                    if app.att_syntax { "AT&T" } else { "Intel" }
                ));
            }

            ui.separator();

            // Architecture selector
            ui.label("Arch:");
            let old_arch = app.target_arch;
            egui::ComboBox::from_id_salt("arch_combo")
                .selected_text(format!("{:?}", app.target_arch))
                .show_ui(ui, |ui| {
                    for &arch in &TargetArch::ALL {
                        ui.selectable_value(&mut app.target_arch, arch, format!("{:?}", arch));
                    }
                });
            if old_arch != app.target_arch {
                app.log(&format!(
                    "[Arch] Switched architecture to {:?}",
                    app.target_arch
                ));
                app.do_assemble();
            }
        });
    });
}
