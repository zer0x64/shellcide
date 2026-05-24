use eframe::egui::{self, Color32};
use crate::app::{ShellcideApp, TargetArch};
use crate::debugger::DebuggerCommand;

fn generate_syscall_boilerplate(info: &crate::syscalls::SyscallInfo, att_syntax: bool) -> String {
    let mut s = String::new();
    let comment_char = if att_syntax { "/* " } else { "; " };
    let comment_end = if att_syntax { " */" } else { "" };
    
    // Add comment detailing the signature
    s.push_str(&format!("{}{}(", comment_char, info.entry_point));
    let args_str: Vec<&str> = info.args.iter().map(|arg| arg.arg_type).collect();
    s.push_str(&args_str.join(", "));
    s.push_str(&format!("){}\n", comment_end));

    if att_syntax {
        // AT&T Syntax
        s.push_str(&format!("movq ${}, %rax\n", info.nr));
        for arg in info.args {
            let reg_name = arg.reg; // has % prefix
            let arg_name = crate::ui::syscalls::extract_arg_name(arg.arg_type);
            s.push_str(&format!("movq $0, {}  /* {} */\n", reg_name, arg_name));
        }
        s.push_str("syscall\n");
    } else {
        // Intel Syntax
        s.push_str(&format!("mov rax, {}\n", info.nr));
        for arg in info.args {
            // Strip % prefix
            let reg_name = arg.reg.strip_prefix('%').unwrap_or(arg.reg);
            let arg_name = crate::ui::syscalls::extract_arg_name(arg.arg_type);
            s.push_str(&format!("mov {}, 0  ; {}\n", reg_name, arg_name));
        }
        s.push_str("syscall\n");
    }
    s
}

pub fn render_editor_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("Shellcode Editor");
    ui.separator();

    let text_edit_id = egui::Id::new("code_editor_text_edit");

    // Intercept Tab / Shift-Tab key presses if the editor has focus
    let has_focus = ui.ctx().memory(|mem| mem.has_focus(text_edit_id));
    let mut tab_pressed = false;
    let mut shift_pressed = false;

    if has_focus {
        ui.ctx().input_mut(|i| {
            i.events.retain(|event| {
                if let egui::Event::Key { key: egui::Key::Tab, pressed: true, modifiers, .. } = event {
                    tab_pressed = true;
                    shift_pressed = modifiers.shift;
                    false // Consume (prevent default focus shift or raw tab insert)
                } else {
                    true
                }
            });
        });
    }

    if tab_pressed {
        let mut state = egui::widgets::text_edit::TextEditState::load(ui.ctx(), text_edit_id).unwrap_or_default();
        if let Some(range) = state.cursor.char_range() {
            let start_char = range.primary.index.min(range.secondary.index);
            let end_char = range.primary.index.max(range.secondary.index);

            if start_char == end_char && !shift_pressed {
                // Case A: Tab with no selection -> Insert 4 spaces at cursor
                let byte_idx = app.code_input.char_indices()
                    .nth(start_char)
                    .map(|(i, _)| i)
                    .unwrap_or(app.code_input.len());
                app.code_input.insert_str(byte_idx, "    ");

                // Update cursor index forward by 4 chars
                let new_idx = start_char + 4;
                let new_range = egui::text::CCursorRange::two(
                    egui::text::CCursor::new(new_idx),
                    egui::text::CCursor::new(new_idx),
                );
                state.cursor.set_char_range(Some(new_range));
                state.store(ui.ctx(), text_edit_id);
            } else {
                // Case B: Tab with selection, or Shift-Tab (always affects whole lines)
                // 1. Gather starting character indices of every line
                let mut line_starts = Vec::new();
                let mut current_char_idx = 0;
                line_starts.push(0);
                for c in app.code_input.chars() {
                    current_char_idx += 1;
                    if c == '\n' {
                        line_starts.push(current_char_idx);
                    }
                }

                let get_line_idx = |char_idx: usize, line_starts: &[usize]| -> usize {
                    match line_starts.binary_search(&char_idx) {
                        Ok(idx) => idx,
                        Err(idx) => idx - 1,
                    }
                };

                let start_line = get_line_idx(start_char, &line_starts);
                let mut end_line = get_line_idx(end_char, &line_starts);
                // If end cursor lands exactly at start of a new line, exclude it from block indentation
                if end_line > start_line && end_char == line_starts[end_line] {
                    end_line -= 1;
                }

                // 2. Re-build code line-by-line while calculating selection cursor shift deltas
                let lines: Vec<&str> = app.code_input.split('\n').collect();
                let mut new_lines = Vec::new();
                let mut delta_at_start = 0;
                let mut delta_at_end = 0;
                let mut char_idx_before_line = 0;

                for (i, line) in lines.iter().enumerate() {
                    let line_chars = line.chars().count();
                    let line_start_idx = char_idx_before_line;

                    if i >= start_line && i <= end_line {
                        if !shift_pressed {
                            // Indent: add 4 spaces
                            new_lines.push(format!("    {}", line));
                            if start_char >= line_start_idx {
                                delta_at_start += 4;
                            }
                            if end_char >= line_start_idx {
                                delta_at_end += 4;
                            }
                        } else {
                            // Outdent: strip up to 4 spaces or 1 tab
                            let mut chars_to_remove = 0;
                            for c in line.chars().take(4) {
                                if c == ' ' {
                                    chars_to_remove += 1;
                                } else if c == '\t' {
                                    chars_to_remove += 1;
                                    break;
                                } else {
                                    break;
                                }
                            }

                            let rest: String = line.chars().skip(chars_to_remove).collect();
                            new_lines.push(rest);

                            if start_char >= line_start_idx {
                                let max_shift = chars_to_remove.min(start_char - line_start_idx);
                                delta_at_start -= max_shift as i32;
                            }
                            if end_char >= line_start_idx {
                                let max_shift = chars_to_remove.min(end_char - line_start_idx);
                                delta_at_end -= max_shift as i32;
                            }
                        }
                    } else {
                        new_lines.push(line.to_string());
                    }

                    char_idx_before_line += line_chars + 1; // +1 for the newline
                }

                app.code_input = new_lines.join("\n");

                let new_start_char = (start_char as i32 + delta_at_start).max(0) as usize;
                let new_end_char = (end_char as i32 + delta_at_end).max(0) as usize;

                let new_range = if range.primary.index <= range.secondary.index {
                    egui::text::CCursorRange::two(
                        egui::text::CCursor::new(new_start_char),
                        egui::text::CCursor::new(new_end_char),
                    )
                } else {
                    egui::text::CCursorRange::two(
                        egui::text::CCursor::new(new_end_char),
                        egui::text::CCursor::new(new_start_char),
                    )
                };

                state.cursor.set_char_range(Some(new_range));
                state.store(ui.ctx(), text_edit_id);
            }
        }
    }

    // Dynamic drop-zone frame highlighting using animated alpha
    let dnd_alpha = ui.ctx().animate_bool_with_time(
        egui::Id::new("dnd_hover_highlight"),
        ui.ctx().dragged_id().is_some(),
        0.1,
    );

    let frame = if dnd_alpha > 0.0 {
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgba_unmultiplied(0, 206, 201, (dnd_alpha * 10.0) as u8))
            .stroke(egui::Stroke::new(2.0, Color32::from_rgba_unmultiplied(0, 206, 201, (dnd_alpha * 255.0) as u8)))
            .inner_margin(4.0)
    } else {
        egui::Frame::none()
    };

    let (_dnd_resp, dropped_payload) = ui.dnd_drop_zone::<String, _>(frame, |ui| {
        let bad_char_lines = app.bad_char_lines.clone();
        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let mut job = crate::editor::highlight_assembly(ui, string, &bad_char_lines);
            job.wrap.max_width = f32::INFINITY; // Disable wrapping to ensure line-number alignment
            ui.fonts(|f| f.layout_job(job))
        };

        let scroll_height = ui.available_height() - 200.0;
        egui::ScrollArea::vertical().id_salt("editor_scroll").max_height(scroll_height).show(ui, |ui| {
            ui.horizontal(|ui| {
                let row_height = ui.fonts(|f| f.row_height(&egui::FontId::monospace(14.0)));
                let margin_top = 6.0; // Matches standard TextEdit top margin
                
                // Gutter column
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0; // 0.0 spacing to match text rows perfectly
                    ui.add_space(margin_top);
                    
                    let lines_count = app.code_input.lines().count().max(1);
                    let is_native = app.target_arch == TargetArch::X86_64;
                    for i in 0..lines_count {
                        let has_bp = app.editor_breakpoints.contains(&i);
                        
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(60.0, row_height), 
                            egui::Sense::click()
                        );
                        
                        if is_native && response.clicked() {
                            let line_to_inst = app.get_line_to_inst_mapping();
                            let resolved_addr = line_to_inst.get(&i)
                                .and_then(|&idx| app.disassembly.get(idx))
                                .map(|inst| inst.address as usize);

                            let was_present = app.editor_breakpoints.contains(&i);
                            if was_present {
                                app.editor_breakpoints.remove(&i);
                            } else {
                                app.editor_breakpoints.insert(i);
                            }

                            let action = if was_present { "Removed" } else { "Added" };
                            if let Some(addr) = resolved_addr {
                                if was_present {
                                    app.breakpoints.remove(&addr);
                                } else {
                                    app.breakpoints.insert(addr);
                                }
                                app.cmd_tx.send(DebuggerCommand::ToggleBreakpoint(addr, !was_present)).ok();
                                app.log(&format!("[Breakpoint] {} at line {}, 0x{:08X}", action, i + 1, addr));
                            } else {
                                app.log(&format!("[Breakpoint] {} at line {}", action, i + 1));
                            }
                        }
                        
                        let text_color = if is_native && has_bp {
                            Color32::from_rgb(255, 118, 117)
                        } else {
                            Color32::from_rgb(99, 110, 114)
                        };
                        
                        let bp_char = if is_native && has_bp { "● " } else { "  " };
                        let warn_char = if app.bad_char_lines.contains(&i) { "⚠️ " } else { "  " };
                        let label = format!("{}{}{:>2}", bp_char, warn_char, i + 1);
                        
                        ui.painter().text(
                            rect.left_center(),
                            egui::Align2::LEFT_CENTER,
                            label,
                            egui::FontId::monospace(12.0),
                            text_color
                        );
                    }
                });
                
                // The TextEdit
                egui::ScrollArea::horizontal().id_salt("editor_horiz_scroll").show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut app.code_input)
                            .id(text_edit_id)
                            .font(egui::FontId::monospace(14.0))
                            .code_editor()
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .desired_rows(30)
                            .layouter(&mut layouter)
                    );
                });
            });
        });
    });

    if let Some(payload) = dropped_payload {
        let syscall_name = &*payload;
        if let Some(s) = crate::syscalls::SYSCALLS.iter().find(|s| s.name == syscall_name) {
            let boilerplate = generate_syscall_boilerplate(s, app.att_syntax);
            
            // Insert at the cursor or append
            let mut state = egui::widgets::text_edit::TextEditState::load(ui.ctx(), text_edit_id).unwrap_or_default();
            let char_range = state.cursor.char_range();
            
            let inserted_len = boilerplate.chars().count();
            
            let (byte_idx, char_idx) = if let Some(range) = char_range {
                let cursor_idx = range.primary.index;
                let byte_offset = app.code_input.char_indices()
                    .nth(cursor_idx)
                    .map(|(i, _)| i)
                    .unwrap_or(app.code_input.len());
                (byte_offset, cursor_idx)
            } else {
                let total_chars = app.code_input.chars().count();
                (app.code_input.len(), total_chars)
            };
            
            app.code_input.insert_str(byte_idx, &boilerplate);
            
            // Update cursor position to end of inserted boilerplate
            let new_char_idx = char_idx + inserted_len;
            let new_range = egui::text::CCursorRange::two(
                egui::text::CCursor::new(new_char_idx),
                egui::text::CCursor::new(new_char_idx),
            );
            state.cursor.set_char_range(Some(new_range));
            state.store(ui.ctx(), text_edit_id);
            
            app.log(&format!("[Editor] Inserted boilerplate for syscall: {}", syscall_name));
        }
    }

    ui.add_space(8.0);
    ui.collapsing("🚫 Bad Characters", |ui| {
        ui.horizontal(|ui| {
            ui.label("Exclude bytes:");
            ui.add(
                egui::TextEdit::singleline(&mut app.bad_chars_input)
                    .hint_text("e.g. 00 0a 0d 90-ff")
                    .desired_width(180.0)
            );
            if ui.button("Clear").clicked() {
                app.bad_chars_input.clear();
            }
        });
        
        let bad_chars = crate::assembler::parse_bad_characters(&app.bad_chars_input);
        if !bad_chars.is_empty() {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                ui.label(egui::RichText::new("Parsed:").weak());
                let mut sorted_chars: Vec<&u8> = bad_chars.iter().collect();
                sorted_chars.sort();
                for &b in sorted_chars {
                    ui.label(egui::RichText::new(format!("{:02x}", b)).monospace().color(Color32::from_rgb(255, 118, 117)));
                }
            });
        } else {
            ui.colored_label(Color32::GRAY, "No bad characters defined.");
        }
        ui.add_space(4.0);
    });
}
