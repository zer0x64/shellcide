use eframe::egui::{self, Color32};
use crate::app::ShellcideApp;

pub fn render_header_panel(app: &mut ShellcideApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("SHELLCIDE // X86_64 ASSEMBLER IDE & DEBUGGER")
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
        });
    });
}
