use crate::app::{ShellcideApp, TargetArch};
use crate::ui::theme::CYBER_CYAN;
use eframe::egui;

pub fn render_header_panel(app: &mut ShellcideApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let title = if app.target_arch == TargetArch::X86_64 {
                "SHELLCIDE // X86_64 IDE & DEBUGGER".to_string()
            } else {
                format!("SHELLCIDE // {} IDE", app.target_arch.display_name()).to_uppercase()
            };
            ui.heading(
                egui::RichText::new(title)
                    .color(CYBER_CYAN)
                    .strong(),
            );

            ui.separator();

            // File Operations
            ui.label("File:");
            ui.add(egui::TextEdit::singleline(&mut app.active_path).desired_width(120.0));

            if ui.button("📁 Load").clicked() {
                #[cfg(not(target_arch = "wasm32"))]
                match std::fs::read_to_string(&app.active_path) {
                    Ok(content) => {
                        app.code_input = content;
                        app.log(&format!("[File] Loaded file: {}", app.active_path));
                    }
                    Err(e) => app.log(&format!("[File System Error] Failed to load: {}", e)),
                }
                #[cfg(target_arch = "wasm32")]
                load_file_wasm(app.event_tx.clone());
            }

            if ui.button("💾 Save").clicked() {
                #[cfg(not(target_arch = "wasm32"))]
                match std::fs::write(&app.active_path, &app.code_input) {
                    Ok(_) => app.log(&format!("[File] Saved to: {}", app.active_path)),
                    Err(e) => app.log(&format!("[File System Error] Failed to save: {}", e)),
                }
                #[cfg(target_arch = "wasm32")]
                save_file_wasm(&app.active_path, &app.code_input);
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
                    ui.selectable_value(&mut app.target_arch, TargetArch::X86_64, "X86_64");
                    ui.selectable_value(&mut app.target_arch, TargetArch::X86, "X86");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Arm, "Arm");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Thumb, "Thumb");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Aarch64, "Aarch64");
                    ui.selectable_value(&mut app.target_arch, TargetArch::Riscv, "Riscv");
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

#[cfg(target_arch = "wasm32")]
fn save_file_wasm(filename: &str, content: &str) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(content));
    let blob = web_sys::Blob::new_with_str_sequence(&array).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let link = document.create_element("a").unwrap();
    let html_link = link.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
    html_link.set_href(&url);
    html_link.set_download(filename);
    let body = document.body().unwrap();
    body.append_child(&html_link).unwrap();
    html_link.click();
    body.remove_child(&html_link).unwrap();
    let _ = web_sys::Url::revoke_object_url(&url);
}

#[cfg(target_arch = "wasm32")]
fn load_file_wasm(event_tx: flume::Sender<crate::debugger::DebuggerEvent>) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let input = document.create_element("input").unwrap();
    let html_input = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    html_input.set_type("file");
    html_input.set_accept(".s,.asm,.txt,.bin,*");
    let tx_clone = event_tx.clone();
    let input_clone = html_input.clone();
    let on_change = Closure::wrap(Box::new(move |_: web_sys::Event| {
        if let Some(files) = input_clone.files() {
            if let Some(file) = files.get(0) {
                let filename = file.name();
                let file_reader = web_sys::FileReader::new().unwrap();
                let tx_inner = tx_clone.clone();
                let file_reader_clone = file_reader.clone();
                let filename_clone = filename.clone();
                let on_load = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Ok(result) = file_reader_clone.result() {
                        if let Some(content) = result.as_string() {
                            let _ = tx_inner.send(crate::debugger::DebuggerEvent::FileLoaded {
                                filename: filename_clone.clone(),
                                content,
                            });
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                file_reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
                on_load.forget();
                file_reader.read_as_text(&file).unwrap();
            }
        }
    }) as Box<dyn FnMut(_)>);
    html_input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    on_change.forget();
    html_input.click();
}
